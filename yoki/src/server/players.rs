use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::server::game_profile::GameProfile;
use crate::server::packet_registry::PacketRegistry;
use minecraft_packet::ConnectionWriter;
use minecraft_packet::packets::play::keep_alive::ClientboundKeepAlivePacket;
use minecraft_packet::packets::play::system_chat::SystemChatMessagePacket;
use protocol_version::protocol_version::ProtocolVersion;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Debug)]
pub enum Outgoing {
    Packet(Box<PacketRegistry>),
    SetVersion(ProtocolVersion),
}

pub type OutgoingSender = UnboundedSender<Outgoing>;

pub fn spawn_writer_task(mut writer: ConnectionWriter, mut receiver: UnboundedReceiver<Outgoing>) {
    tokio::spawn(async move {
        let mut protocol_version = ProtocolVersion::Any;
        while let Some(outgoing) = receiver.recv().await {
            match outgoing {
                Outgoing::SetVersion(version) => protocol_version = version,
                Outgoing::Packet(packet) => {
                    let raw = match packet.encode_clientbound(protocol_version) {
                        Ok(raw) => raw,
                        Err(err) => {
                            tracing::error!("failed to encode outgoing packet: {err}");
                            break;
                        }
                    };
                    if writer.send_raw(&raw).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
}

pub fn spawn_keep_alive_task(sender: OutgoingSender) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(KEEP_ALIVE_INTERVAL);
        interval.tick().await;

        loop {
            interval.tick().await;
            let packet = PacketRegistry::ClientboundKeepAlive(ClientboundKeepAlivePacket::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|elapsed| elapsed.as_millis() as i64)
                    .unwrap_or_default(),
            ));
            if sender.send(Outgoing::Packet(Box::new(packet))).is_err() {
                break;
            }
        }
    });
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug)]
pub struct ConnectedPlayer {
    entity_id: i32,
    profile: GameProfile,
    protocol_version: ProtocolVersion,
    position: Mutex<PlayerPosition>,
    sender: OutgoingSender,
}

impl ConnectedPlayer {
    pub fn new(
        entity_id: i32,
        profile: GameProfile,
        protocol_version: ProtocolVersion,
        position: PlayerPosition,
        sender: OutgoingSender,
    ) -> Self {
        Self {
            entity_id,
            profile,
            protocol_version,
            position: Mutex::new(position),
            sender,
        }
    }

    pub const fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub const fn profile(&self) -> &GameProfile {
        &self.profile
    }

    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    pub fn position(&self) -> PlayerPosition {
        *self.position.lock().expect("position lock poisoned")
    }

    pub fn update_position(&self, update: impl FnOnce(&mut PlayerPosition)) -> PlayerPosition {
        let mut position = self.position.lock().expect("position lock poisoned");
        update(&mut position);
        *position
    }

    pub fn send_packet(&self, packet: PacketRegistry) {
        let _ = self.sender.send(Outgoing::Packet(Box::new(packet)));
    }

    pub fn send(&self, message: impl Into<String>) {
        self.send_packet(PacketRegistry::SystemChatMessage(
            SystemChatMessagePacket::text(message),
        ))
    }
}

#[derive(Debug, Default)]
pub struct PlayerRegistry {
    players: RwLock<HashMap<i32, Arc<ConnectedPlayer>>>,
    next_entity_id: AtomicI32,
}

impl PlayerRegistry {
    pub fn allocate_entity_id(&self) -> i32 {
        self.next_entity_id.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn register(&self, player: Arc<ConnectedPlayer>) -> Vec<Arc<ConnectedPlayer>> {
        tracing::debug!(
            "registering player {} (entity {})",
            player.profile().username(),
            player.entity_id()
        );
        let mut players = self.players.write().expect("players lock poisoned");
        let others: Vec<_> = players.values().cloned().collect();
        players.insert(player.entity_id(), player);
        others
    }

    pub fn unregister(&self, entity_id: i32) -> Option<Arc<ConnectedPlayer>> {
        self.players
            .write()
            .expect("players lock poisoned")
            .remove(&entity_id)
    }

    pub fn get(&self, entity_id: i32) -> Option<Arc<ConnectedPlayer>> {
        self.players
            .read()
            .expect("players lock poisoned")
            .get(&entity_id)
            .cloned()
    }

    pub fn others(&self, entity_id: i32) -> Vec<Arc<ConnectedPlayer>> {
        self.players
            .read()
            .expect("players lock poisoned")
            .values()
            .filter(|player| player.entity_id() != entity_id)
            .cloned()
            .collect()
    }

    pub fn get_size(&self) -> usize {
        self.players.read().expect("players lock poisoned").len()
    }

    pub fn all(&self) -> Vec<Arc<ConnectedPlayer>> {
        self.players
            .read()
            .expect("players lock poisoned")
            .values()
            .cloned()
            .collect()
    }
}
