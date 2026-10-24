use yoki_binutils::data_types::NbtTag;
use yoki_identifier::Identifier;

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub entry_id: Identifier,
    pub data: Option<NbtTag>,
}

#[derive(Debug, Clone)]
pub struct RegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, Clone)]
pub struct TagEntry {
    pub name: Identifier,
    pub entries: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct RegistryTags {
    pub registry: Identifier,
    pub tags: Vec<TagEntry>,
}
