use crate::binary_error::BinaryError;

pub trait WriteBytes {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError>;
}

#[derive(Default)]
pub struct BinaryWriter {
    buf: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write<T: WriteBytes>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.write(self)
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    pub fn write_all(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub fn extend(&mut self, bytes: impl AsRef<[u8]>) {
        self.write_all(bytes.as_ref());
    }

    pub fn write_byte_array(&mut self, value: &[u8]) -> Result<(), BinaryError> {
        value.to_vec().write(self)
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}

pub(crate) fn write_be<const N: usize>(writer: &mut BinaryWriter, bytes: [u8; N]) {
    writer.write_all(&bytes);
}
