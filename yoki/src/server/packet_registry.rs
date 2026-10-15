use minecraft_packet::packets::play::block_changed_ack::BlockChangedAckPacket;
use minecraft_packet::packets::play::block_update::BlockUpdatePacket;
use minecraft_packet::packets::play::chat_command::ChatCommandPacket;
use minecraft_packet::packets::play::chat_message::ChatMessagePacket;
use minecraft_packet::packets::play::command_suggestion::CommandSuggestionPacket;
use minecraft_packet::packets::play::command_suggestions::CommandSuggestionsPacket;
use minecraft_packet::packets::play::commands::CommandsPacket;
use minecraft_packet::packets::play::level_chunk_with_light::LevelChunkWithLightPacket;
use minecraft_packet::packets::play::player_action::PlayerActionPacket;
use minecraft_packet::packets::play::set_carried_item::SetCarriedItemPacket;
use minecraft_packet::packets::play::set_creative_mode_slot::SetCreativeModeSlotPacket;
use minecraft_packet::packets::play::system_chat::SystemChatMessagePacket;
use minecraft_packet::packets::play::use_item_on::UseItemOnPacket;
use minecraft_packet::packets::play::{
    add_entity::AddEntityPacket,
    animate::AnimatePacket,
    entity_position_sync::EntityPositionSyncPacket,
    game_event::GameEventPacket,
    keep_alive::{ClientboundKeepAlivePacket, ServerboundKeepAlivePacket},
    move_player::{
        MovePlayerPosPacket, MovePlayerPosRotPacket, MovePlayerRotPacket,
        MovePlayerStatusOnlyPacket,
    },
    player_abilities::{PlayerAbilitiesPacket, ServerboundPlayerAbilitiesPacket},
    player_info_remove::PlayerInfoRemovePacket,
    player_info_update::PlayerInfoUpdatePacket,
    remove_entities::RemoveEntitiesPacket,
    rotate_head::RotateHeadPacket,
    set_chunk_cache_center::SetChunkCacheCenterPacket,
    swing::SwingPacket,
    synchronize_player_position::SynchronizePlayerPositionPacket,
};
use minecraft_packet::packets::{
    HandshakePacket, PingRequestPacket, PongResponsePacket, StatusRequestPacket,
    StatusResponsePacket,
    configuration::{
        acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
        client_information::ClientInformationPacket,
        finish_configuration::FinishConfigurationPacket,
        known_packs::{SelectKnownPacksClientboundPacket, SelectKnownPacksPacket},
        plugin_message::PluginMessagePacket,
        registry_data::RegistryDataPacket,
        update_enabled_features::UpdateEnabledFeaturesPacket,
        update_tags::UpdateTagsPacket,
    },
    login::{
        cookie_response_login::CookieResponseLoginPacket,
        encryption_response::EncryptionResponsePacket, login_acknowledged::LoginAcknowledgedPacket,
        login_plugin_response::LoginPluginResponsePacket, login_start::LoginStartPacket,
        login_success::LoginSuccessPacket,
    },
    play::login::LoginPacket,
};
use yoki_macros::PacketReport;

use crate::{
    ServerState,
    batch::Batch,
    server::{PacketHandler, packet_handler::PacketHandlerError},
};

#[derive(Debug, PacketReport)]
pub enum PacketRegistry {
    #[protocol_id(state = "handshake", bound = "serverbound", id = "minecraft:intention")]
    Handshake(HandshakePacket),

    #[protocol_id(
        state = "status",
        bound = "serverbound",
        id = "minecraft:status_request"
    )]
    StatusRequest(StatusRequestPacket),

    #[protocol_id(
        state = "status",
        bound = "clientbound",
        id = "minecraft:status_response"
    )]
    StatusResponse(StatusResponsePacket),

    #[protocol_id(state = "status", bound = "serverbound", id = "minecraft:ping_request")]
    PingRequest(PingRequestPacket),

    #[protocol_id(
        state = "status",
        bound = "clientbound",
        id = "minecraft:pong_response"
    )]
    PongResponse(PongResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:hello")]
    LoginStart(LoginStartPacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        id = "minecraft:login_finished"
    )]
    LoginSuccess(LoginSuccessPacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:key")]
    EncryptionResponse(EncryptionResponsePacket),

    #[protocol_id(
        state = "login",
        bound = "serverbound",
        id = "minecraft:custom_query_answer"
    )]
    LoginPluginResponse(LoginPluginResponsePacket),

    #[protocol_id(
        state = "login",
        bound = "serverbound",
        id = "minecraft:login_acknowledged"
    )]
    LoginAcknowledged(LoginAcknowledgedPacket),

    #[protocol_id(
        state = "login",
        bound = "serverbound",
        id = "minecraft:cookie_response"
    )]
    CookieResponseLogin(CookieResponseLoginPacket),

    #[protocol_id(
        state = "configuration",
        bound = "serverbound",
        id = "minecraft:client_information"
    )]
    ClientInformation(ClientInformationPacket),

    #[protocol_id(
        state = "configuration",
        bound = "serverbound",
        id = "minecraft:custom_payload"
    )]
    PluginMessage(PluginMessagePacket),

    #[protocol_id(
        state = "configuration",
        bound = "serverbound",
        id = "minecraft:finish_configuration"
    )]
    AcknowledgeFinishConfiguration(AcknowledgeFinishConfigurationPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        id = "minecraft:finish_configuration"
    )]
    FinishConfiguration(FinishConfigurationPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        id = "minecraft:select_known_packs"
    )]
    SelectKnownPacksClientbound(SelectKnownPacksClientboundPacket),

    #[protocol_id(
        state = "configuration",
        bound = "serverbound",
        id = "minecraft:select_known_packs"
    )]
    SelectKnownPacks(SelectKnownPacksPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        id = "minecraft:update_enabled_features"
    )]
    UpdateEnabledFeatures(UpdateEnabledFeaturesPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        id = "minecraft:update_tags"
    )]
    UpdateTags(UpdateTagsPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        id = "minecraft:registry_data"
    )]
    RegistryData(RegistryDataPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:login")]
    Login(LoginPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:player_abilities"
    )]
    PlayerAbilities(PlayerAbilitiesPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:player_position"
    )]
    SynchronizePlayerPosition(SynchronizePlayerPositionPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:game_event")]
    GameEvent(GameEventPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:set_chunk_cache_center"
    )]
    SetChunkCacheCenter(SetChunkCacheCenterPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:level_chunk_with_light"
    )]
    LevelChunkWithLight(LevelChunkWithLightPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:keep_alive")]
    ClientboundKeepAlive(ClientboundKeepAlivePacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:keep_alive")]
    ServerboundKeepAlive(ServerboundKeepAlivePacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:player_info_update"
    )]
    PlayerInfoUpdate(PlayerInfoUpdatePacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:player_info_remove"
    )]
    PlayerInfoRemove(PlayerInfoRemovePacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:add_entity")]
    AddEntity(AddEntityPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:remove_entities"
    )]
    RemoveEntities(RemoveEntitiesPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:entity_position_sync"
    )]
    EntityPositionSync(EntityPositionSyncPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:rotate_head")]
    RotateHead(RotateHeadPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:animate")]
    Animate(AnimatePacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:move_player_pos"
    )]
    MovePlayerPos(MovePlayerPosPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:move_player_pos_rot"
    )]
    MovePlayerPosRot(MovePlayerPosRotPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:move_player_rot"
    )]
    MovePlayerRot(MovePlayerRotPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:move_player_status_only"
    )]
    MovePlayerStatusOnly(MovePlayerStatusOnlyPacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:swing")]
    Swing(SwingPacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:chat")]
    ChatMessage(ChatMessagePacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:system_chat")]
    SystemChatMessage(SystemChatMessagePacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:chat_command")]
    ChatCommand(ChatCommandPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:commands")]
    Commands(CommandsPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:player_abilities"
    )]
    ServerboundPlayerAbilities(ServerboundPlayerAbilitiesPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:block_update")]
    BlockUpdate(BlockUpdatePacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:block_changed_ack"
    )]
    BlockChangedAck(BlockChangedAckPacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:player_action")]
    PlayerAction(PlayerActionPacket),

    #[protocol_id(state = "play", bound = "serverbound", id = "minecraft:use_item_on")]
    UseItemOn(UseItemOnPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:set_creative_mode_slot"
    )]
    SetCreativeModeSlot(SetCreativeModeSlotPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:set_carried_item"
    )]
    SetCarriedItem(SetCarriedItemPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        id = "minecraft:command_suggestion"
    )]
    CommandSuggestion(CommandSuggestionPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        id = "minecraft:command_suggestions"
    )]
    CommandSuggestions(CommandSuggestionsPacket),
}

impl PacketHandler for PacketRegistry {
    fn handle(
        &self,
        client_state: &mut super::ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        match self {
            Self::Handshake(packet) => packet.handle(client_state, server_state),
            Self::StatusRequest(packet) => packet.handle(client_state, server_state),
            Self::PingRequest(packet) => packet.handle(client_state, server_state),
            Self::LoginStart(packet) => packet.handle(client_state, server_state),
            Self::EncryptionResponse(packet) => packet.handle(client_state, server_state),
            Self::LoginPluginResponse(packet) => packet.handle(client_state, server_state),
            Self::LoginAcknowledged(packet) => packet.handle(client_state, server_state),
            Self::CookieResponseLogin(packet) => packet.handle(client_state, server_state),
            Self::ClientInformation(packet) => packet.handle(client_state, server_state),
            Self::PluginMessage(packet) => packet.handle(client_state, server_state),
            Self::SelectKnownPacks(packet) => packet.handle(client_state, server_state),
            Self::AcknowledgeFinishConfiguration(packet) => {
                packet.handle(client_state, server_state)
            }
            Self::ServerboundKeepAlive(packet) => packet.handle(client_state, server_state),
            Self::MovePlayerPos(packet) => packet.handle(client_state, server_state),
            Self::MovePlayerPosRot(packet) => packet.handle(client_state, server_state),
            Self::MovePlayerRot(packet) => packet.handle(client_state, server_state),
            Self::MovePlayerStatusOnly(packet) => packet.handle(client_state, server_state),
            Self::Swing(packet) => packet.handle(client_state, server_state),
            Self::ChatMessage(packet) => packet.handle(client_state, server_state),
            Self::ChatCommand(packet) => packet.handle(client_state, server_state),
            Self::ServerboundPlayerAbilities(packet) => packet.handle(client_state, server_state),
            Self::CommandSuggestion(packet) => packet.handle(client_state, server_state),
            _ => Ok(Batch::new()),
        }
    }
}
