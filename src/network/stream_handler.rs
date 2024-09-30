use super::{packet::PacketId, packet_handler::*, protocol::Protocol};
use crate::Result;
use tokio::net::TcpStream;
use tracing::info;

pub async fn handle_stream(stream: &mut TcpStream) -> Result<()> {
    info!("Spawned task");

    loop {
        let packet_byte = stream.read_byte().await?;

        match PacketId::n(packet_byte as u8).unwrap() {
            PacketId::Handshake => handle_handshake(stream).await?,
            PacketId::Login => handle_login(stream).await?,
            _ => unreachable!(),
        }
    }
}
