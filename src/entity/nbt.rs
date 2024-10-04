use parry3d::na::Vector3;
use serde::{Deserialize, Serialize};

use crate::item::nbt::ItemData;

#[derive(Deserialize, Serialize)]
pub struct EntityData {
    #[serde(rename = "Age")]
    pub age: i16,
    #[serde(rename = "Air")]
    pub air: i16,
    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,
    #[serde(rename = "Fire")]
    pub fire: i16,
    #[serde(rename = "Health")]
    pub health: i16,
    #[serde(rename = "Item")]
    pub item: Option<ItemData>,
    #[serde(rename = "Motion")]
    pub motion: Vector3<f64>,
    #[serde(rename = "Pos")]
    pub pos: Vector3<f64>,
    #[serde(rename = "Rotation")]
    pub rotation: Vector3<f32>,
    #[serde(rename = "OnGround")]
    pub on_ground: bool,
    pub id: String,
}
