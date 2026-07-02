use minecraft_packet::packets::configuration::known_packs::KnownPack;
use protocol_version::protocol_version::ProtocolVersion;

pub fn known_packs_for_version(version: ProtocolVersion) -> Vec<KnownPack> {
  version
    .data()
    .known_packs()
    .iter()
    .map(|pack_version| KnownPack {
      namespace: "minecraft".to_string(),
      id: "core".to_string(),
      version: (*pack_version).to_string(),
    })
    .collect()
}

pub fn client_accepted_known_pack(
  client_packs: &[KnownPack],
  version: ProtocolVersion,
) -> bool {
  let expected_versions = version.data().known_packs();
  client_packs.iter().any(|pack| {
    pack.namespace == "minecraft"
      && pack.id == "core"
      && expected_versions.contains(&pack.version.as_str())
  })
}
