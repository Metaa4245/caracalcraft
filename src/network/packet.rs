#![allow(dead_code)]
use super::protocol::Protocol;
use crate::Result;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub trait Packet<T> {
    async fn read(stream: &mut TcpStream) -> Result<T>;
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

#[derive(Serialize, Deserialize)]
pub struct KeepAlive {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub protocol_version: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTime {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub time: i64,
}

// TODO: here Packet5PlayerInventory

#[derive(Serialize, Deserialize)]
pub struct SpawnPosition {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Flying {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub on_ground: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerPosition {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub stance: f64,
    pub moving: bool,
    pub on_ground: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerLook {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub yaw: f32,
    pub pitch: f32,
    pub rotating: bool,
    pub on_ground: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerLookMove {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
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

#[derive(Serialize, Deserialize)]
pub struct BlockDig {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub x: i32,
    pub y: i8,
    pub z: i32,
    pub status: i8,
    pub face: i8,
}

#[derive(Serialize, Deserialize)]
pub struct Place {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub id: i16,
    pub x: i32,
    pub y: i8,
    pub z: i32,
    pub direction: i8,
}

#[derive(Serialize, Deserialize)]
pub struct BlockItemSwitch {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub id: i16,
}

#[derive(Serialize, Deserialize)]
pub struct AddToInventory {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub item_id: i16,
    pub count: i8,
    pub item_damage: i16,
}

#[derive(Serialize, Deserialize)]
pub struct ArmAnimation {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub animate: i8,
}

#[derive(Serialize, Deserialize)]
pub struct NamedEntitySpawn {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub current_item: i16,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i8,
    pub pitch: i8,
}

#[derive(Serialize, Deserialize)]
pub struct PickupSpawn {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
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

#[derive(Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct Collect {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub collected_entity_id: i32,
    pub collector_entity_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct VehicleSpawn {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub entity_type: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MobSpawn {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub entity_type: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
}

#[derive(Serialize, Deserialize)]
pub struct DestroyEntity {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RelEntityMove {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Serialize, Deserialize)]
pub struct EntityLook {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub yaw: i8,
    pub pitch: i8,
}

#[derive(Serialize, Deserialize)]
pub struct RelEntityMoveLook {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub yaw: i8,
    pub pitch: i8,
}

#[derive(Serialize, Deserialize)]
pub struct EntityTeleport {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub entity_id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
}

#[derive(Serialize, Deserialize)]
pub struct PreChunk {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub x: i32,
    pub z: i32,
    pub mode: bool,
}

#[derive(Serialize, Deserialize)]
pub struct KickDisconnect {
    #[serde(skip_deserializing)]
    pub packet_id: i8,
    pub reason: String,
}

impl Packet<Self> for KeepAlive {
    async fn read(_stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::KeepAlive as i8,
        })
    }
}

impl Packet<Self> for Login {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Login as i8,
            protocol_version: stream.read_i32().await?,
            username: stream.read_string().await?,
            password: stream.read_string().await?,
        })
    }
}

impl Packet<Self> for Handshake {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Handshake as i8,
            username: stream.read_string().await?,
        })
    }
}

impl Packet<Self> for Chat {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Chat as i8,
            message: stream.read_string().await?,
        })
    }
}

impl Packet<Self> for UpdateTime {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::UpdateTime as i8,
            time: stream.read_long().await?,
        })
    }
}

// TODO: here Packet5PlayerInventory

impl Packet<Self> for SpawnPosition {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::SpawnPosition as i8,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
        })
    }
}

impl Packet<Self> for Flying {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Flying as i8,
            on_ground: stream.read_bool().await?,
        })
    }
}

impl Packet<Self> for PlayerPosition {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::PlayerPosition as i8,
            x: stream.read_double().await?,
            y: stream.read_double().await?,
            stance: stream.read_double().await?,
            z: stream.read_double().await?,
            moving: true,
            on_ground: stream.read_bool().await?,
        })
    }
}

impl Packet<Self> for PlayerLook {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::PlayerLook as i8,
            yaw: stream.read_float().await?,
            pitch: stream.read_float().await?,
            rotating: true,
            on_ground: stream.read_bool().await?,
        })
    }
}

impl Packet<Self> for PlayerLookMove {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::PlayerLookMove as i8,
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
}

impl Packet<Self> for BlockDig {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::BlockDig as i8,
            status: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_byte().await?,
            z: stream.read_int().await?,
            face: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for Place {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Place as i8,
            id: stream.read_short().await?,
            x: stream.read_int().await?,
            y: stream.read_byte().await?,
            z: stream.read_int().await?,
            direction: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for BlockItemSwitch {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::BlockItemSwitch as i8,
            entity_id: stream.read_int().await?,
            id: stream.read_short().await?,
        })
    }
}

impl Packet<Self> for AddToInventory {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::AddToInventory as i8,
            item_id: stream.read_short().await?,
            count: stream.read_byte().await?,
            item_damage: stream.read_short().await?,
        })
    }
}

impl Packet<Self> for ArmAnimation {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::ArmAnimation as i8,
            entity_id: stream.read_int().await?,
            animate: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for NamedEntitySpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::NamedEntitySpawn as i8,
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
}

impl Packet<Self> for PickupSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::PickupSpawn as i8,
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
}

impl Packet<Self> for Collect {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Collect as i8,
            collected_entity_id: stream.read_int().await?,
            collector_entity_id: stream.read_int().await?,
        })
    }
}

impl Packet<Self> for VehicleSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::VehicleSpawn as i8,
            entity_id: stream.read_int().await?,
            entity_type: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
        })
    }
}

impl Packet<Self> for MobSpawn {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::MobSpawn as i8,
            entity_id: stream.read_int().await?,
            entity_type: stream.read_byte().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
            yaw: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for DestroyEntity {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::DestroyEntity as i8,
            entity_id: stream.read_int().await?,
        })
    }
}

impl Packet<Self> for Entity {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::Entity as i8,
            entity_id: stream.read_int().await?,
        })
    }
}

impl Packet<Self> for RelEntityMove {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::RelEntityMove as i8,
            entity_id: stream.read_int().await?,
            x: stream.read_byte().await?,
            y: stream.read_byte().await?,
            z: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for EntityLook {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::EntityLook as i8,
            entity_id: stream.read_int().await?,
            yaw: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for RelEntityMoveLook {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::RelEntityMoveLook as i8,
            entity_id: stream.read_int().await?,
            x: stream.read_byte().await?,
            y: stream.read_byte().await?,
            z: stream.read_byte().await?,
            yaw: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for EntityTeleport {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::EntityTeleport as i8,
            entity_id: stream.read_int().await?,
            x: stream.read_int().await?,
            y: stream.read_int().await?,
            z: stream.read_int().await?,
            yaw: stream.read_byte().await?,
            pitch: stream.read_byte().await?,
        })
    }
}

impl Packet<Self> for PreChunk {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::PreChunk as i8,
            x: stream.read_int().await?,
            z: stream.read_int().await?,
            mode: stream.read_bool().await?,
        })
    }
}

impl Packet<Self> for KickDisconnect {
    async fn read(stream: &mut TcpStream) -> Result<Self> {
        Ok(Self {
            packet_id: PacketId::KickDisconnect as i8,
            reason: stream.read_string().await?,
        })
    }
}
