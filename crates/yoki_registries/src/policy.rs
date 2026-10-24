use protocol_version::protocol_version::ProtocolVersion;

pub fn should_omit_known_pack_entry_data(
    version: ProtocolVersion,
    client_accepted_known_pack: bool,
) -> bool {
    client_accepted_known_pack
        && matches!(
            version.data(),
            ProtocolVersion::V26_1 | ProtocolVersion::V26_2
        )
}
