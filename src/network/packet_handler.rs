use crate::Result;
use tokio::net::TcpStream;
use tracing::info;

use super::packet::*;

pub async fn handle_handshake(stream: &mut TcpStream) -> Result<()> {
    let handshake = Handshake::read(stream).await?;
    info!("{} handshaking", handshake.username);

    let response = Handshake {
        username: "-".to_owned(),
    };
    response.write(stream).await?;

    Ok(())
}

pub async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let login = Login::read(stream).await?;
    info!("{} logging in", login.username);

    if login.protocol_version != 2 {
        info!("Rejected {} (different version)", login.username);
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

    info!("{} logged in", login.username);
    Ok(())
}
