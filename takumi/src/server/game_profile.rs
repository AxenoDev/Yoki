use uuid::Uuid;

#[derive(Clone)]
pub struct GameProfile {
    username: String,
    uuid: Uuid,
}

impl GameProfile {
    pub fn new(username: &str, uuid: Uuid) -> Self {
        let username = username
            .get(..16)
            .map_or_else(|| username.to_string(), std::string::ToString::to_string);
        Self { username, uuid }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn set_name(&mut self, new_name: &str) {
        let new_name = new_name
            .get(..16)
            .map_or_else(|| new_name.to_string(), std::string::ToString::to_string);
        self.username = new_name;
    }
}
