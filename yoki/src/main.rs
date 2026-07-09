use clap::builder::styling::AnsiColor;
use clap::Parser;
use minecraft_packet::ConnectionReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fmt, fs, io};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use yoki::{
    handlers::play::handle_player_disconnect, server::{
        client_state::ClientState, packet_handler::PacketHandlerError, packet_registry::PacketRegistry,
        players::spawn_writer_task, PacketHandler,
    },
    Cli,
    ServerState,
};
use yoki_binutils::ProtocolError;

fn rotate_latest_log(server_dir: &PathBuf) -> io::Result<()> {
    let logs_dir = server_dir.join("logs");
    let logs = Path::new(&logs_dir);
    fs::create_dir_all(logs)?;

    let latest = logs.join("latest.log");
    if latest.exists() {
        let date = chrono::Local::now().format("%Y-%m-%d");
        let mut index = 1;
        loop {
            let archive = logs.join(format!("{date}-{index}.log"));
            if !archive.exists() {
                fs::rename(&latest, &archive)?;
                break;
            }
            index += 1;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let server_dir = &cli.server_dir;
    rotate_latest_log(server_dir).expect("Failed to rotate log");
    let file =
        fs::File::create(server_dir.join("logs/latest.log")).expect("Failed to create log file");
    let (file_writer, _guard) = tracing_appender::non_blocking(file);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(YokiFormat)
                .with_ansi(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(YokiFormat)
                .with_writer(file_writer)
                .with_ansi(false),
        )
        .init();
    let server_state =
        Arc::new(ServerState::load(server_dir.join("config")).expect("failed to load config"));
    let addr = server_state.bind();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind on {addr}"));

    tracing::info!("Yoki listening on {addr}");

    loop {
        let (socket, client_addr) = listener.accept().await.unwrap();
        let server_state = Arc::clone(&server_state);

        tokio::spawn(async move {
            tracing::debug!("connection opened from {client_addr}");
            if let Err(err) = handle_connection(socket, server_state).await {
                tracing::warn!("connection error from {client_addr}: {err}");
            }
            tracing::debug!("connection closed for {client_addr}");
        });
    }
}

async fn handle_connection(
    socket: tokio::net::TcpStream,
    server_state: Arc<ServerState>,
) -> Result<(), ProtocolError> {
    let (reader, writer) = minecraft_packet::split(socket);
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    spawn_writer_task(writer, receiver);

    let mut client_state = ClientState::new();
    client_state.set_sender(sender);

    let result = run_read_loop(reader, &mut client_state, &server_state).await;

    if let Some(entity_id) = client_state.entity_id() {
        handle_player_disconnect(&server_state, entity_id);
    }

    result
}

async fn run_read_loop(
    mut reader: ConnectionReader,
    client_state: &mut ClientState,
    server_state: &ServerState,
) -> Result<(), ProtocolError> {
    loop {
        let raw = reader.receive().await?;
        let packet = match PacketRegistry::decode_serverbound(
            client_state.protocol_version(),
            client_state.serverbound_state(),
            &raw,
        ) {
            Ok(packet) => packet,
            Err(ProtocolError::UnknownPacket { id, conn }) => {
                tracing::debug!("ignoring unknown packet 0x{id:02X} (state {conn:?})");
                continue;
            }
            Err(err) => return Err(err),
        };

        let should_disconnect = matches!(packet, PacketRegistry::PingRequest(_));

        let batch = match packet.handle(client_state, server_state) {
            Ok(batch) => batch,
            Err(PacketHandlerError::InvalidState(message, should_warn)) => {
                if should_warn {
                    tracing::warn!("{message}");
                } else {
                    tracing::debug!("closing connection: {message}");
                }
                break;
            }
            Err(PacketHandlerError::Custom(message)) => {
                return Err(ProtocolError::Io(message));
            }
        };

        batch.execute(client_state).await?;

        if should_disconnect {
            break;
        }
    }

    Ok(())
}

struct YokiFormat;

impl<S, N> FormatEvent<S, N> for YokiFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let time = chrono::Local::now().format("%H:%M:%S");
        let level = *event.metadata().level();

        if writer.has_ansi_escapes() {
            let color = match level {
                Level::ERROR => AnsiColor::Red,
                Level::WARN => AnsiColor::Yellow,
                Level::INFO => AnsiColor::Green,
                Level::DEBUG => AnsiColor::Cyan,
                Level::TRACE => AnsiColor::White,
            };
            let ansi_color = color.on_default();
            write!(
                writer,
                "\x1b[90m[{time}\x1b[0m {ansi_color}{level}\x1b[90m]:\x1b[0m "
            )?;
        } else {
            write!(writer, "[{time} {level}]: ")?;
        }

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
