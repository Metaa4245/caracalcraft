use super::{
    packet::PacketId,
    packet_handler::{handle_handshake, handle_login},
    protocol::Protocol,
};
use crate::Result;
use tokio::net::TcpStream;

#[allow(clippy::cast_sign_loss)]
pub async fn handle_stream(stream: &mut TcpStream) -> Result<()> {
    loop {
        let packet_byte = stream.read_byte().await?;

        match PacketId::n(packet_byte as u8).unwrap() {
            PacketId::Handshake => handle_handshake(stream).await?,
            PacketId::Login => handle_login(stream).await?,
            _ => unreachable!(),
        }
    }
}
