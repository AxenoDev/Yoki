use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

const NODE_TYPE_LITERAL: u8 = 0x01;
const NODE_TYPE_ARGUMENT: u8 = 0x02;
const FLAG_EXECUTABLE: u8 = 0x04;

#[derive(Debug, Clone)]
pub struct CommandNode {
    pub kind: NodeKind,
    pub executable: bool,
    pub children: Vec<i32>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Root,
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser_id: i32,
        properties: ArgumentProperties,
    },
}

#[derive(Debug, Clone)]
pub enum ArgumentProperties {
    None,
    StringWord,
    StringGreedy,
    Int { min: Option<i32>, max: Option<i32> },
    Float { min: Option<f32>, max: Option<f32> },
    SinglePlayer,
}

#[derive(Debug, Clone)]
pub struct CommandsPacket {
    pub nodes: Vec<CommandNode>,
    pub root_index: i32,
}

impl PacketMeta for CommandsPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for CommandsPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        VarInt::new(self.nodes.len() as i32).write(writer)?;
        for node in &self.nodes {
            write_node(writer, node)?;
        }
        VarInt::new(self.root_index).write(writer)?;
        Ok(())
    }
}

fn write_node(writer: &mut BinaryWriter, node: &CommandNode) -> Result<(), ProtocolError> {
    let node_type = match &node.kind {
        NodeKind::Root => 0,
        NodeKind::Literal { .. } => NODE_TYPE_LITERAL,
        NodeKind::Argument { .. } => NODE_TYPE_ARGUMENT,
    };
    let mut flags = node_type;
    if node.executable {
        flags |= FLAG_EXECUTABLE;
    }
    writer.write_byte(flags);

    node.children
        .iter()
        .map(|child| VarInt::new(*child))
        .collect::<Vec<_>>()
        .write(writer)?;

    match &node.kind {
        NodeKind::Root => {}
        NodeKind::Literal { name } => {
            name.write(writer)?;
        }
        NodeKind::Argument {
            name,
            parser_id,
            properties,
        } => {
            name.write(writer)?;
            VarInt::new(*parser_id).write(writer)?;
            write_properties(writer, properties)?;
        }
    }

    Ok(())
}

fn write_properties(
    writer: &mut BinaryWriter,
    properties: &ArgumentProperties,
) -> Result<(), ProtocolError> {
    match properties {
        ArgumentProperties::None => {}
        ArgumentProperties::StringWord => VarInt::new(0).write(writer)?,
        ArgumentProperties::StringGreedy => VarInt::new(2).write(writer)?,
        ArgumentProperties::Int { min, max } => {
            let mut flags = 0u8;
            if min.is_some() {
                flags |= 0x01;
            }
            if max.is_some() {
                flags |= 0x02;
            }
            writer.write_byte(flags);
            if let Some(min) = min {
                min.write(writer)?;
            }
            if let Some(max) = max {
                max.write(writer)?;
            }
        }
        ArgumentProperties::Float { min, max } => {
            let mut flags = 0u8;
            if min.is_some() {
                flags |= 0x01;
            }
            if max.is_some() {
                flags |= 0x02;
            }
            writer.write_byte(flags);
            if let Some(min) = min {
                min.write(writer)?;
            }
            if let Some(max) = max {
                max.write(writer)?;
            }
        }
        ArgumentProperties::SinglePlayer => writer.write_byte(0x03),
    }
    Ok(())
}
