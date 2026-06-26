pub trait EncodePacket: Sized {
    fn encode(&self) -> Result<Vec<u8>, String>;
}
