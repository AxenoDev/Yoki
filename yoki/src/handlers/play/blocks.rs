use minecraft_packet::packets::play::block_changed_ack::BlockChangedAckPacket;
use minecraft_packet::packets::play::block_update::BlockUpdatePacket;
use minecraft_packet::packets::play::data::BlockPos;
use minecraft_packet::packets::play::player_action::PlayerActionPacket;
use minecraft_packet::packets::play::set_carried_item::SetCarriedItemPacket;
use minecraft_packet::packets::play::set_creative_mode_slot::SetCreativeModeSlotPacket;
use minecraft_packet::packets::play::use_item_on::UseItemOnPacket;

use crate::batch::Batch;
use crate::server::packet_handler::PacketHandlerError;
use crate::server::packet_registry::PacketRegistry;
use crate::server::players::ConnectedPlayer;
use crate::server::world::AIR_STATE;
use crate::server::{ClientState, PacketHandler};
use crate::ServerState;

fn change_block(
    server_state: &ServerState,
    pos: BlockPos,
    state: i32,
    sequence: i32,
    actor: &ConnectedPlayer,
) {
    server_state.world().set_block(pos, state);
    for player in server_state.players().all() {
        player.send_packet(PacketRegistry::BlockUpdate(BlockUpdatePacket::new(
            pos, state,
        )));
    }
    actor.send_packet(PacketRegistry::BlockChangedAck(BlockChangedAckPacket::new(
        sequence,
    )));
}

fn sender(
    client_state: &ClientState,
    server_state: &ServerState,
) -> Option<std::sync::Arc<ConnectedPlayer>> {
    let entity_id = client_state.entity_id()?;
    server_state.players().get(entity_id)
}

impl PacketHandler for PlayerActionPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        if self.is_break()
            && let Some(actor) = sender(client_state, server_state)
        {
            tracing::debug!(
                "entity {} breaks block at {:?}",
                actor.entity_id(),
                self.pos
            );
            change_block(server_state, self.pos, AIR_STATE, self.sequence, &actor);
        }
        Ok(Batch::new())
    }
}

impl PacketHandler for UseItemOnPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let Some(actor) = sender(client_state, server_state) else {
            return Ok(Batch::new());
        };

        let placed = actor.held_item_id().and_then(|item_id| {
            server_state
                .registry_store()
                .item_block_state_id(actor.protocol_version(), item_id)
        });

        match placed {
            Some(state) => {
                let target = self.pos.offset_by_face(self.face);
                tracing::debug!("entity {} places block at {target:?}", actor.entity_id());
                change_block(server_state, target, state, self.sequence, &actor);
            }
            None => {
                actor.send_packet(PacketRegistry::BlockChangedAck(BlockChangedAckPacket::new(
                    self.sequence,
                )));
            }
        }
        Ok(Batch::new())
    }
}

impl PacketHandler for SetCreativeModeSlotPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        if let Some(actor) = sender(client_state, server_state) {
            actor.set_creative_slot(self.slot, self.item_id);
        }
        Ok(Batch::new())
    }
}

impl PacketHandler for SetCarriedItemPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        if let Some(actor) = sender(client_state, server_state) {
            actor.set_carried_index(self.slot);
        }
        Ok(Batch::new())
    }
}
