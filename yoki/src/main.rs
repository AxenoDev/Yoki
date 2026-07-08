use std::sync::Arc;

use clap::Parser;
use minecraft_packet::ConnectionReader;
use yoki::{
    Cli, ServerState,
    handlers::play::handle_player_disconnect,
    server::{
        PacketHandler, client_state::ClientState, packet_handler::PacketHandlerError,
        packet_registry::PacketRegistry, players::spawn_writer_task,
    },
};
use yoki_binutils::ProtocolError;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let server_state = Arc::new(ServerState::load(&cli.config_dir).expect("failed to load config"));
    let addr = server_state.bind();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind on {addr}"));

    println!("Yoki listening on {addr}");

    loop {
        let (socket, client_addr) = listener.accept().await.unwrap();
        let server_state = Arc::clone(&server_state);

        tokio::spawn(async move {
            if let Err(err) = handle_connection(socket, server_state).await {
                eprintln!("{client_addr}: {err}");
            }
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
            Err(ProtocolError::UnknownPacket { .. }) => continue,
            Err(err) => return Err(err),
        };

        let should_disconnect = matches!(packet, PacketRegistry::PingRequest(_));

        let batch = match packet.handle(client_state, server_state) {
            Ok(batch) => batch,
            Err(PacketHandlerError::InvalidState(message, should_warn)) => {
                if should_warn {
                    eprintln!("{message}");
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
