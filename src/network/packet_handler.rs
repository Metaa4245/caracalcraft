use crate::Result;
use tokio::net::TcpStream;

use super::{
    packet::{Handshake, KickDisconnect, Login, Packet},
    protocol::Protocol,
    serde::serializer,
};

pub async fn handle_handshake(stream: &mut TcpStream) -> Result<()> {
    let _ = Handshake::read(stream).await?;

    let response = Handshake {
        username: "-".to_owned(),
    };
    stream.write_bytes(serializer::to_bytes(&response)?).await?;

    Ok(())
}

pub async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let login = Login::read(stream).await?;

    if login.protocol_version != 2 {
        let disconnect = KickDisconnect {
            reason: "Protocol version isn't 2! (use a1.1.2_01)".to_owned(),
        };
        stream
            .write_bytes(serializer::to_bytes(&disconnect)?)
            .await?;
        return Ok(());
    }

    let response = Login {
        password: String::new(),
        username: String::new(),
        protocol_version: 0,
    };
    stream.write_bytes(serializer::to_bytes(&response)?).await?;

    Ok(())
}
