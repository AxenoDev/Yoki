use strum_macros::{Display, AsRefStr};

#[derive(Copy, Debug, PartialEq, Clone, Eq, Hash, Display, AsRefStr)]
pub enum DirectionBound {
    #[strum(serialize = "clientbound")]
    Clientbound,
    #[strum(serialize = "serverbound")]
    Serverbound,
}

#[derive(Copy, Debug, PartialEq, Clone, Default, Eq, Hash, Display, AsRefStr)]
pub enum State {
    #[default]
    #[strum(serialize = "handshake")]
    Handshaking,
    #[strum(serialize = "status")]
    Status,
    #[strum(serialize = "login")]
    Login,
    #[strum(serialize = "configuration")]
    Configuration,
    #[strum(serialize = "play")]
    Play,
    #[strum(serialize = "transfer")]
    Transfer,
}
