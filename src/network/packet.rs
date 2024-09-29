#![allow(dead_code)]
use super::{packet_id::PacketId, protocol::Protocol};
use crate::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub trait Packet<T> {
    async fn read(stream: &mut TcpStream) -> Result<T>;
    async fn write(&self, stream: &mut TcpStream) -> Result<()>;
}

pub struct KeepAlive {}

pub struct Login {
    protocol_version: i32,
    username: String,
    password: String,
}

pub struct Handshake {
    username: String,
}

pub struct Chat {
    message: String,
}

pub struct UpdateTime {
    time: i64,
}

// TODO: here Packet5PlayerInventory

pub struct SpawnPosition {
    x: i32,
    y: i32,
    z: i32,
}

pub struct KickDisconnect {
    reason: String,
}

pub struct Flying {
    on_ground: bool,
}

impl Packet<Self> for KeepAlive {
    async fn read(_stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {})
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::KeepAlive as i8).await?;

        Ok(())
    }
}

impl Packet<Self> for Login {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            protocol_version: stream.read_i32().await?,
            username: stream.read_string().await?,
            password: stream.read_string().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Login as i8).await?;
        stream.write_string(self.username.clone()).await?;
        stream.write_string(self.password.clone()).await?;

        Ok(())
    }
}

impl Packet<Self> for Handshake {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            username: stream.read_string().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Handshake as i8).await?;
        stream.write_string(self.username.clone()).await?;

        Ok(())
    }
}

impl Packet<Self> for Chat {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            message: stream.read_string().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Chat as i8).await?;
        stream.write_string(self.message.clone()).await?;

        Ok(())
    }
}

impl Packet<Self> for UpdateTime {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            time: stream.read_long().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::UpdateTime as i8).await?;
        stream.write_long(self.time).await?;

        Ok(())
    }
}

// TODO: here Packet5PlayerInventory

impl Packet<Self> for SpawnPosition {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::SpawnPosition as i8).await?;
        stream.write_int(self.x).await?;
        stream.write_int(self.y).await?;
        stream.write_int(self.z).await?;

        Ok(())
    }
}

impl Packet<Self> for Flying {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            on_ground: stream.read_bool().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Flying as i8).await?;
        stream.write_bool(self.on_ground).await?;

        Ok(())
    }
}

impl Packet<Self> for KickDisconnect {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            reason: stream.read_string().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::KickDisconnect as i8).await?;
        stream.write_string(self.reason.clone()).await?;

        Ok(())
    }
}
