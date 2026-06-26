pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write_varint(&mut self, mut value: i32) {
        loop {
            if (value & !0x7F) == 0 {
                self.buf.push(value as u8);
                return;
            }

            self.buf.push(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_varint(value.len() as i32);
        self.buf.extend_from_slice(value.as_bytes());
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_uuid(&mut self, value: &uuid::Uuid) {
        self.buf.extend_from_slice(value.as_bytes());
    }

    pub fn write_bool(&mut self, value: bool) {
        self.buf.push(if value { 1 } else { 0 });
    }

    pub fn write_i8(&mut self, value: i8) {
        self.buf.push(value as u8);
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buf.push(value);
    }

    pub fn write_byte_array(&mut self, value: &[u8]) {
        self.write_varint(value.len() as i32);
        self.buf.extend_from_slice(value);
    }

    pub fn extend(&mut self, bytes: impl AsRef<[u8]>) {
        self.buf.extend_from_slice(bytes.as_ref());
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

impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}
