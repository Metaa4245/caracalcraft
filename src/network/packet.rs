#![allow(dead_code)]
use super::protocol::Protocol;
use crate::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub trait Packet<T> {
    async fn read(stream: &mut TcpStream) -> Result<T>;
    async fn write(&self, stream: &mut TcpStream) -> Result<()>;
}

#[derive(enumn::N, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PacketId {
    KeepAlive,
    Login,
    Handshake,
    Chat,
    UpdateTime,
    PlayerInventory,
    SpawnPosition,
    Flying = 10,
    PlayerPosition,
    PlayerLook,
    PlayerLookMove,
    BlockDig,
    Place,
    BlockItemSwitch,
    AddToInventory,
    ArmAnimation,
    NamedEntitySpawn = 20,
    PickupSpawn,
    Collect,
    VehicleSpawn,
    MobSpawn,
    DestroyEntity = 29,
    Entity,
    RelEntityMove,
    EntityLook,
    RelEntityMoveLook,
    EntityTeleport,
    PreChunk = 50,
    MapChunk,
    MultiBlockChange,
    BlockChange,
    ComplexEntity = 59,
    KickDisconnect = 255,
}

pub struct KeepAlive {}

pub struct Login {
    pub protocol_version: i32,
    pub username: String,
    pub password: String,
}

pub struct Handshake {
    pub username: String,
}

pub struct Chat {
    pub message: String,
}

pub struct UpdateTime {
    pub time: i64,
}

// TODO: here Packet5PlayerInventory

pub struct SpawnPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct Flying {
    pub on_ground: bool,
}

pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub stance: f64,
    pub moving: bool,
    pub on_ground: bool,
}

pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
    pub rotating: bool,
    pub on_ground: bool,
}

pub struct PlayerLookMove {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub stance: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub moving: bool,
    pub rotating: bool,
    pub on_ground: bool,
}

pub struct BlockDig {
    pub x: i32,
    pub y: i8,
    pub z: i32,
    pub status: i8,
    pub face: i8,
}

pub struct Place {
    pub id: i16,
    pub x: i32,
    pub y: i8,
    pub z: i32,
    pub direction: i8,
}

pub struct BlockItemSwitch {
    pub entity_id: i32,
    pub id: i16,
}

pub struct AddToInventory {
    pub item_id: i16,
    pub count: i8,
    pub item_damage: i16,
}

pub struct ArmAnimation {
    pub entity_id: i32,
    pub animate: i8,
}

pub struct NamedEntitySpawn {
    pub entity_id: i32,
    pub current_item: i16,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i8,
    pub pitch: i8,
}

pub struct PickupSpawn {
    pub entity_id: i32,
    pub item_id: i16,
    pub count: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i8,
    pub pitch: i8,
    pub roll: i8,
}

pub struct Collect {
    pub collected_entity_id: i32,
    pub collector_entity_id: i32,
}

pub struct VehicleSpawn {
    pub entity_id: i32,
    pub entity_type: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct MobSpawn {
    pub entity_id: i32,
    pub entity_type: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
}

pub struct DestroyEntity {
    pub entity_id: i32,
}

pub struct Entity {
    pub entity_id: i32,
}

pub struct RelEntityMove {
    pub entity_id: i32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub struct KickDisconnect {
    pub reason: String,
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

impl Packet<Self> for PlayerPosition {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            x: stream.read_double().await?,
            y: stream.read_double().await?,
            stance: stream.read_double().await?,
            z: stream.read_double().await?,
            moving: true,
            on_ground: stream.read_bool().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::PlayerPosition as i8).await?;
        stream.write_double(self.x).await?;
        stream.write_double(self.y).await?;
        stream.write_double(self.stance).await?;
        stream.write_double(self.z).await?;
        stream.write_bool(self.on_ground).await?;

        Ok(())
    }
}

impl Packet<Self> for PlayerLook {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            yaw: stream.read_float().await?,
            pitch: stream.read_float().await?,
            rotating: true,
            on_ground: stream.read_bool().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::PlayerLook as i8).await?;
        stream.write_float(self.yaw).await?;
        stream.write_float(self.pitch).await?;
        stream.write_bool(self.on_ground).await?;

        Ok(())
    }
}

impl Packet<Self> for PlayerLookMove {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            x: stream.read_double().await?,
            y: stream.read_double().await?,
            stance: stream.read_double().await?,
            z: stream.read_double().await?,
            yaw: stream.read_float().await?,
            pitch: stream.read_float().await?,
            on_ground: stream.read_bool().await?,
            moving: true,
            rotating: true,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::PlayerLookMove as i8).await?;
        stream.write_double(self.x).await?;
        stream.write_double(self.y).await?;
        stream.write_double(self.stance).await?;
        stream.write_double(self.z).await?;
        stream.write_float(self.yaw).await?;
        stream.write_float(self.pitch).await?;
        stream.write_bool(self.on_ground).await?;

        Ok(())
    }
}

impl Packet<Self> for BlockDig {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            status: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_byte().await?,
            z: stream.read_int().await?,
            face: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::BlockDig as i8).await?;
        stream.write_byte(self.status).await?;
        stream.write_int(self.x).await?;
        stream.write_byte(self.y).await?;
        stream.write_int(self.z).await?;
        stream.write_byte(self.face).await?;

        Ok(())
    }
}

impl Packet<Self> for Place {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            id: stream.read_short().await?,
            x: stream.read_int().await?,
            y: stream.read_byte().await?,
            z: stream.read_int().await?,
            direction: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Place as i8).await?;
        stream.write_short(self.id).await?;
        stream.write_int(self.x).await?;
        stream.write_byte(self.y).await?;
        stream.write_int(self.z).await?;
        stream.write_byte(self.direction).await?;

        Ok(())
    }
}

impl Packet<Self> for BlockItemSwitch {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            id: stream.read_short().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::BlockItemSwitch as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_short(self.id).await?;

        Ok(())
    }
}

impl Packet<Self> for AddToInventory {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            item_id: stream.read_short().await?,
            count: stream.read_byte().await?,
            item_damage: stream.read_short().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::AddToInventory as i8).await?;
        stream.write_short(self.item_id).await?;
        stream.write_byte(self.count).await?;
        stream.write_short(self.item_damage).await?;

        Ok(())
    }
}

impl Packet<Self> for ArmAnimation {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            animate: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::ArmAnimation as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_byte(self.animate).await?;

        Ok(())
    }
}

impl Packet<Self> for NamedEntitySpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            name: stream.read_string().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
            rotation: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
            current_item: stream.read_short().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::NamedEntitySpawn as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_string(self.name.clone()).await?;
        stream.write_int(self.x).await?;
        stream.write_int(self.y).await?;
        stream.write_int(self.z).await?;
        stream.write_byte(self.rotation).await?;
        stream.write_byte(self.pitch).await?;
        stream.write_short(self.current_item).await?;

        Ok(())
    }
}

impl Packet<Self> for PickupSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            item_id: stream.read_short().await?,
            count: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
            rotation: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
            roll: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::PickupSpawn as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_short(self.item_id).await?;
        stream.write_byte(self.count).await?;
        stream.write_int(self.x).await?;
        stream.write_int(self.y).await?;
        stream.write_int(self.z).await?;
        stream.write_byte(self.rotation).await?;
        stream.write_byte(self.pitch).await?;
        stream.write_byte(self.roll).await?;

        Ok(())
    }
}

impl Packet<Self> for Collect {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            collected_entity_id: stream.read_int().await?,
            collector_entity_id: stream.read_int().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Collect as i8).await?;
        stream.write_int(self.collected_entity_id).await?;
        stream.write_int(self.collector_entity_id).await?;

        Ok(())
    }
}

impl Packet<Self> for VehicleSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            entity_type: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::VehicleSpawn as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_byte(self.entity_type).await?;
        stream.write_int(self.x).await?;
        stream.write_int(self.y).await?;
        stream.write_int(self.z).await?;

        Ok(())
    }
}

impl Packet<Self> for MobSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            entity_type: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
            yaw: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::MobSpawn as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_byte(self.entity_type).await?;
        stream.write_int(self.x).await?;
        stream.write_int(self.y).await?;
        stream.write_int(self.z).await?;
        stream.write_byte(self.yaw).await?;
        stream.write_byte(self.pitch).await?;

        Ok(())
    }
}

impl Packet<Self> for DestroyEntity {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::DestroyEntity as i8).await?;
        stream.write_int(self.entity_id).await?;

        Ok(())
    }
}

impl Packet<Self> for Entity {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::Entity as i8).await?;
        stream.write_int(self.entity_id).await?;

        Ok(())
    }
}

impl Packet<Self> for RelEntityMove {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            entity_id: stream.read_int().await?,
            x: stream.read_byte().await?,
            y: stream.read_byte().await?,
            z: stream.read_byte().await?,
        })
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_byte(PacketId::RelEntityMove as i8).await?;
        stream.write_int(self.entity_id).await?;
        stream.write_byte(self.x).await?;
        stream.write_byte(self.y).await?;
        stream.write_byte(self.z).await?;

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
