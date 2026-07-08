use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

use crate::packet::{OutgoingPacket, RawPacket};

pub fn split(stream: TcpStream) -> (ConnectionReader, ConnectionWriter) {
    let (read_half, write_half) = stream.into_split();
    (
        ConnectionReader { stream: read_half },
        ConnectionWriter { stream: write_half },
    )
}

pub struct ConnectionReader {
    stream: OwnedReadHalf,
}

impl ConnectionReader {
    pub async fn receive(&mut self) -> Result<RawPacket, ProtocolError> {
        let length = self.read_varint().await? as usize;
        let mut data = vec![0u8; length];
        self.stream.read_exact(&mut data).await?;

        let mut pos = 0;
        let packet_id = read_varint_from_slice(&data, &mut pos)?;
        let payload = data.split_off(pos);

        Ok(RawPacket {
            id: packet_id,
            payload,
        })
    }

    async fn read_varint(&mut self) -> Result<i32, ProtocolError> {
        let mut result = 0i32;
        let mut num_read = 0;

        loop {
            let mut byte = [0u8; 1];
            self.stream.read_exact(&mut byte).await?;

            let b = byte[0];
            result |= ((b & 0x7F) as i32) << (7 * num_read);
            num_read += 1;

            if num_read > 5 {
                return Err(ProtocolError::VarIntTooBig);
            }

            if b & 0x80 == 0 {
                break;
            }
        }

        Ok(result)
    }
}

pub struct ConnectionWriter {
    stream: OwnedWriteHalf,
}

impl ConnectionWriter {
    pub async fn send<P: OutgoingPacket>(
        &mut self,
        id: i32,
        packet: &P,
    ) -> Result<(), ProtocolError> {
        let payload = packet.encode_with_id(id)?;
        self.send_framed(&payload).await
    }

    pub async fn send_raw(&mut self, packet: &RawPacket) -> Result<(), ProtocolError> {
        let mut body = BinaryWriter::new();
        VarInt::from(packet.id).write(&mut body)?;
        body.extend(&packet.payload);
        self.send_framed(&body.into_inner()).await
    }

    async fn send_framed(&mut self, payload: &[u8]) -> Result<(), ProtocolError> {
        let mut frame = BinaryWriter::new();
        VarInt::from(payload.len() as i32).write(&mut frame)?;
        frame.extend(payload);

        self.stream.write_all(&frame.into_inner()).await?;
        Ok(())
    }
}

fn read_varint_from_slice(data: &[u8], pos: &mut usize) -> Result<i32, ProtocolError> {
    let mut num_read = 0;
    let mut result = 0i32;

    loop {
        if *pos >= data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let byte = data[*pos];
        *pos += 1;

        result |= ((byte & 0x7F) as i32) << (7 * num_read);
        num_read += 1;

        if num_read > 5 {
            return Err(ProtocolError::VarIntTooBig);
        }

        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok(result)
}
