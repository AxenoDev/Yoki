use std::collections::BTreeMap;

use crate::binary_error::BinaryError;
use crate::binary_writer::{BinaryWriter, WriteBytes};

#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<NbtTag>),
    Compound(BTreeMap<String, NbtTag>),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NbtWriteOptions {
    pub nameless_root: bool,
    pub dynamic_lists: bool,
}

impl NbtWriteOptions {
    pub const fn network() -> Self {
        Self {
            nameless_root: true,
            dynamic_lists: true,
        }
    }
}

impl NbtTag {
    pub fn byte(value: bool) -> Self {
        Self::Byte(i8::from(value))
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn compound(entries: BTreeMap<String, NbtTag>) -> Self {
        Self::Compound(entries)
    }
}

pub fn write_network_nbt(
    writer: &mut BinaryWriter,
    root: &NbtTag,
    options: NbtWriteOptions,
) -> Result<(), BinaryError> {
    match root {
        NbtTag::Compound(entries) => {
            writer.write_byte(10);
            if !options.nameless_root {
                write_nbt_string(writer, "")?;
            }
            write_compound_payload(writer, entries, options)?;
            Ok(())
        }
        _ => Err(BinaryError::Custom(
            "network NBT root must be a compound".into(),
        )),
    }
}

fn write_compound_payload(
    writer: &mut BinaryWriter,
    entries: &BTreeMap<String, NbtTag>,
    options: NbtWriteOptions,
) -> Result<(), BinaryError> {
    for (name, tag) in entries {
        write_named_tag(writer, name, tag, options)?;
    }
    writer.write_byte(0);
    Ok(())
}

fn write_named_tag(
    writer: &mut BinaryWriter,
    name: &str,
    tag: &NbtTag,
    options: NbtWriteOptions,
) -> Result<(), BinaryError> {
    writer.write_byte(tag_type(tag));
    write_nbt_string(writer, name)?;
    write_tag_payload(writer, tag, options)?;
    Ok(())
}

fn write_tag_payload(
    writer: &mut BinaryWriter,
    tag: &NbtTag,
    options: NbtWriteOptions,
) -> Result<(), BinaryError> {
    match tag {
        NbtTag::Byte(value) => writer.write_byte(*value as u8),
        NbtTag::Short(value) => writer.write_all(&value.to_be_bytes()),
        NbtTag::Int(value) => writer.write_all(&value.to_be_bytes()),
        NbtTag::Long(value) => writer.write_all(&value.to_be_bytes()),
        NbtTag::Float(value) => writer.write_all(&value.to_be_bytes()),
        NbtTag::Double(value) => writer.write_all(&value.to_be_bytes()),
        NbtTag::String(value) => write_nbt_string(writer, value)?,
        NbtTag::List(values) => write_list_payload(writer, values, options)?,
        NbtTag::Compound(entries) => write_compound_payload(writer, entries, options)?,
    }
    Ok(())
}

fn write_list_payload(
    writer: &mut BinaryWriter,
    values: &[NbtTag],
    options: NbtWriteOptions,
) -> Result<(), BinaryError> {
    if values.is_empty() {
        writer.write_byte(0);
        writer.write_all(&0i32.to_be_bytes());
        return Ok(());
    }

    let first_type = tag_type(&values[0]);
    let is_heterogeneous = values.iter().any(|value| tag_type(value) != first_type);

    if is_heterogeneous {
        if !options.dynamic_lists {
            return Err(BinaryError::Custom(
                "heterogeneous NBT lists require dynamic_lists".into(),
            ));
        }

        writer.write_byte(10);
        writer.write_all(&(values.len() as i32).to_be_bytes());
        for value in values {
            writer.write_byte(tag_type(value));
            write_nbt_string(writer, "")?;
            write_tag_payload(writer, value, options)?;
            writer.write_byte(0);
        }
        return Ok(());
    }

    writer.write_byte(first_type);
    writer.write_all(&(values.len() as i32).to_be_bytes());
    for value in values {
        write_tag_payload(writer, value, options)?;
    }
    Ok(())
}

fn write_nbt_string(writer: &mut BinaryWriter, value: &str) -> Result<(), BinaryError> {
    let bytes = value.as_bytes();
    if bytes.len() > i16::MAX as usize {
        return Err(BinaryError::Custom("NBT string too long".into()));
    }
    writer.write_all(&(bytes.len() as u16).to_be_bytes());
    writer.write_all(bytes);
    Ok(())
}

fn tag_type(tag: &NbtTag) -> u8 {
    match tag {
        NbtTag::Byte(_) => 1,
        NbtTag::Short(_) => 2,
        NbtTag::Int(_) => 3,
        NbtTag::Long(_) => 4,
        NbtTag::Float(_) => 5,
        NbtTag::Double(_) => 6,
        NbtTag::String(_) => 8,
        NbtTag::List(_) => 9,
        NbtTag::Compound(_) => 10,
    }
}

pub fn json_to_nbt(value: &serde_json::Value) -> Option<NbtTag> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(value) => Some(NbtTag::byte(*value)),
        serde_json::Value::Number(number) => {
            if let Some(value) = number.as_i64() {
                if (-128..=127).contains(&value) {
                    Some(NbtTag::Byte(value as i8))
                } else if (-2147483648..=2147483647).contains(&value) {
                    Some(NbtTag::Int(value as i32))
                } else {
                    Some(NbtTag::Long(value))
                }
            } else if let Some(value) = number.as_f64() {
                if value.fract() == 0.0 && (-2147483648.0..=2147483647.0).contains(&value) {
                    Some(NbtTag::Int(value as i32))
                } else {
                    Some(NbtTag::Double(value))
                }
            } else {
                None
            }
        }
        serde_json::Value::String(value) => Some(NbtTag::String(value.clone())),
        serde_json::Value::Array(values) => Some(NbtTag::List(
            values.iter().filter_map(json_to_nbt).collect(),
        )),
        serde_json::Value::Object(map) => {
            let mut entries = BTreeMap::new();
            for (key, value) in map {
                if let Some(tag) = json_to_nbt(value) {
                    entries.insert(key.clone(), tag);
                }
            }
            Some(NbtTag::Compound(entries))
        }
    }
}

impl WriteBytes for NbtTag {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        write_network_nbt(writer, self, NbtWriteOptions::network())
    }
}
