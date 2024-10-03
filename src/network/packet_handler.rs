use crate::Result;
use tokio::net::TcpStream;

use super::packet::*;

pub async fn handle_handshake(stream: &mut TcpStream) -> Result<()> {
    let _ = Handshake::read(stream).await?;

    let response = Handshake {
        username: "-".to_owned(),
    };
    response.write(stream).await?;

    Ok(())
}

pub async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let login = Login::read(stream).await?;

    if login.protocol_version != 2 {
        let disconnect = KickDisconnect {
            reason: "Protocol version isn't 2! (use a1.1.2_01)".to_owned(),
        };
        disconnect.write(stream).await?;
        return Ok(());
    }

    let response = Login {
        password: String::new(),
        username: String::new(),
        protocol_version: 0,
    };
    response.write(stream).await?;

    Ok(())
}
