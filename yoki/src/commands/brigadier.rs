use minecraft_packet::packets::play::commands::{CommandNode, CommandsPacket, NodeKind};
use protocol_version::protocol_version::ProtocolVersion;

use crate::commands::CommandRegistry;
use crate::registry::RegistryStore;

pub fn build_commands_packet(
    registry: &CommandRegistry,
    registry_store: &RegistryStore,
    version: ProtocolVersion,
) -> CommandsPacket {
    let mut nodes = vec![CommandNode {
        kind: NodeKind::Root,
        executable: false,
        children: Vec::new(),
    }];

    for command in registry.iter() {
        let specs = command.args();

        let literal_index = nodes.len() as i32;
        nodes.push(CommandNode {
            kind: NodeKind::Literal {
                name: command.name().to_string(),
            },
            executable: specs.iter().all(|spec| !spec.required),
            children: Vec::new(),
        });
        nodes[0].children.push(literal_index);

        let mut parent = literal_index as usize;
        for (index, spec) in specs.iter().enumerate() {
            let parser_id = registry_store
                .command_argument_type_id(version, spec.kind.parser_name())
                .unwrap_or_else(|| {
                    panic!(
                        "unknown command argument parser {} for {}",
                        spec.kind.parser_name(),
                        version.humanize()
                    )
                });

            let node_index = nodes.len() as i32;
            nodes.push(CommandNode {
                kind: NodeKind::Argument {
                    name: spec.name.to_string(),
                    parser_id,
                    properties: spec.kind.properties(),
                },
                executable: specs[index + 1..].iter().all(|next| !next.required),
                children: Vec::new(),
            });
            nodes[parent].children.push(node_index);
            parent = node_index as usize;
        }
    }

    CommandsPacket {
        nodes,
        root_index: 0,
    }
}
