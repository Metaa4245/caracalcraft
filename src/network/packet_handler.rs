use crate::Result;
use tokio::net::TcpStream;

use super::{
    packet::{Handshake, KickDisconnect, Login, Packet, PacketId},
    protocol::Protocol,
    serde::serializer,
};

pub async fn handle_handshake(stream: &mut TcpStream) -> Result<()> {
    let _ = Handshake::read(stream).await?;

    let response = Handshake {
        packet_id: PacketId::Handshake as i8,
        username: "-".to_owned(),
    };
    stream.write_bytes(serializer::to_bytes(&response)?).await?;

    Ok(())
}

pub async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let login = Login::read(stream).await?;

    if login.protocol_version != 2 {
        let disconnect = KickDisconnect {
            packet_id: PacketId::KickDisconnect as i8,
            reason: "Protocol version isn't 2! (use a1.1.2_01)".to_owned(),
        };
        stream
            .write_bytes(serializer::to_bytes(&disconnect)?)
            .await?;
        return Ok(());
    }

    let response = Login {
        packet_id: PacketId::Login as i8,
        password: String::new(),
        username: String::new(),
        protocol_version: 0,
    };
    stream.write_bytes(serializer::to_bytes(&response)?).await?;

    Ok(())
}
