use yoki_macros::Pvn;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Pvn)]
#[repr(i32)]
pub enum ProtocolVersion {
    #[default]
    #[pvn(known_packs = ["26.2"])]
    V26_2 = 776,
    #[pvn(known_packs = ["26.1", "26.1.1", "26.1.2"])]
    V26_1 = 775,

    Any = -1,
    #[pvn(packets = Any)]
    Unsupported = -2,
}

impl ProtocolVersion {
    pub const fn protocol_number(&self) -> i32 {
        *self as i32
    }

    pub const fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported)
    }
}
