use minecraft_packet::packets::play::player_abilities::{
    ABILITY_ALLOW_FLYING, ABILITY_CREATIVE_MODE, ABILITY_FLYING, ABILITY_INVULNERABLE,
    PlayerAbilitiesPacket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub const fn id(self) -> u8 {
        match self {
            Self::Survival => 0,
            Self::Creative => 1,
            Self::Adventure => 2,
            Self::Spectator => 3,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Survival => "survival",
            Self::Creative => "creative",
            Self::Adventure => "adventure",
            Self::Spectator => "spectator",
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        match input.to_ascii_lowercase().as_str() {
            "survival" | "s" | "0" => Some(Self::Survival),
            "creative" | "c" | "1" => Some(Self::Creative),
            "adventure" | "a" | "2" => Some(Self::Adventure),
            "spectator" | "sp" | "3" => Some(Self::Spectator),
            _ => None,
        }
    }

    pub const fn allows_flying(self) -> bool {
        matches!(self, Self::Creative | Self::Spectator)
    }

    pub fn abilities(self, flying: bool) -> PlayerAbilitiesPacket {
        let mut flags = 0i8;
        match self {
            Self::Creative => {
                flags |= ABILITY_INVULNERABLE | ABILITY_ALLOW_FLYING | ABILITY_CREATIVE_MODE
            }
            Self::Spectator => flags |= ABILITY_INVULNERABLE | ABILITY_ALLOW_FLYING,
            Self::Survival | Self::Adventure => {}
        }
        if self == Self::Spectator || (self.allows_flying() && flying) {
            flags |= ABILITY_FLYING;
        }
        PlayerAbilitiesPacket::new(flags)
    }
}
