use parry3d::na::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "Air")]
    pub air: i16,
    #[serde(rename = "AttackTime")]
    pub attack_time: i16,
    #[serde(rename = "DeathTime")]
    pub death_time: i16,
    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,
    #[serde(rename = "Fire")]
    pub fire: i16,
    #[serde(rename = "Health")]
    pub health: i16,
    #[serde(rename = "HurtTime")]
    pub hurt_time: i16,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<i8>,
    #[serde(rename = "Motion")]
    pub motion: Vector3<f64>,
    #[serde(rename = "Pos")]
    pub pos: Vector3<f64>,
    #[serde(rename = "Rotation")]
    pub rotation: Vector3<f32>,
    #[serde(rename = "OnGround")]
    pub on_ground: bool,
}
