use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Level {
    #[serde(rename = "Data")]
    pub data: LevelData,
}

#[derive(Deserialize, Serialize)]
pub struct LevelData {
    #[serde(rename = "LastPlayed")]
    pub last_played: i64,
    #[serde(rename = "RandomSeed")]
    pub random_seed: i64,
    #[serde(rename = "SizeOnDisk")]
    pub size_on_disk: i64,
    #[serde(rename = "SnowCovered")]
    pub snow_covered: bool,
    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,
    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,
    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,
    #[serde(rename = "Time")]
    pub time: i64,
}
