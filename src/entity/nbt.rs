use macaw::{DVec3, Vec3};
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
    pub motion: DVec3,
    #[serde(rename = "Pos")]
    pub pos: DVec3,
    #[serde(rename = "Rotation")]
    pub rotation: Vec3,
    #[serde(rename = "OnGround")]
    pub on_ground: bool,
    pub id: String,
}
