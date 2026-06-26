use std::sync::Arc;

use minecraft_packet::Connection;
use takumi::server::{
    PacketHandler, client_state::ClientState, packet_handler::PacketHandlerError,
    packet_registry::PacketRegistry, server_state::ServerState,
};
use takumi_binutils::ProtocolError;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:25565";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind on port 25565");

    let server_state = Arc::new(ServerState::default());

    println!("Takumi listening on {addr}");

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
    let mut conn = Connection::new(socket);
    let mut client_state = ClientState::new();

    loop {
        let raw = conn.receive().await?;
        let packet = PacketRegistry::decode_serverbound(client_state.serverbound_state(), &raw)?;

        let should_disconnect = matches!(packet, PacketRegistry::PingRequest(_));

        let batch = match packet.handle(&mut client_state, &server_state) {
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

        batch.execute(&mut conn, &mut client_state).await?;

        if should_disconnect {
            break;
        }
    }

    Ok(())
}
