use radix_fmt::radix_36;
use serde::{Deserialize, Serialize};

use crate::entity::{nbt::EntityData, tile::nbt::TileEntityData};

#[derive(Deserialize, Serialize)]
pub struct Region {
    #[serde(rename = "Level")]
    pub data: RegionData,
}

#[derive(Deserialize, Serialize)]
pub struct RegionData {
    #[serde(rename = "TerrainPopulated")]
    pub terrain_populated: bool,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    #[serde(rename = "xPos")]
    pub x: i32,
    #[serde(rename = "zPos")]
    pub z: i32,

    #[serde(rename = "Entities")]
    pub entities: Vec<EntityData>,
    #[serde(rename = "TileEntities")]
    pub tile_entities: Vec<TileEntityData>,
    #[serde(rename = "BlockLight")]
    pub block_light: Vec<i8>,
    #[serde(rename = "Blocks")]
    pub blocks: Vec<i8>,
    #[serde(rename = "Data")]
    pub data: Vec<i8>,
    #[serde(rename = "HeightMap")]
    pub height_map: Vec<i8>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Vec<i8>,
}

impl Region {
    pub fn file_name(&self) -> String {
        let x = radix_36(self.data.x).to_string();
        let z = radix_36(self.data.z).to_string();

        format!("c.{x}.{z}.dat")
    }
}
