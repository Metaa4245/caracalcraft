use super::{packet::PacketId, packet_handler::*, protocol::Protocol};
use crate::Result;
use tokio::net::TcpStream;
use tracing::{info, warn};

pub async fn handle_stream(stream: &mut TcpStream) -> Result<()> {
    info!("Spawned task");

    loop {
        let packet_byte = stream.read_byte().await?;
        let packet_id = PacketId::n(packet_byte as u8);

        if packet_id.is_none() {
            warn!("Unknown packet {packet_byte}");
            continue;
        }
        let packet_id = packet_id.unwrap();

        match packet_id {
            PacketId::Handshake => handle_handshake(stream).await?,
            PacketId::Login => handle_login(stream).await?,
            _ => todo!(),
        }
    }
}
