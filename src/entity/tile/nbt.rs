use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TileEntityData {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
