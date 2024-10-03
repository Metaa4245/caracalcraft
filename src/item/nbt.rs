use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ItemData {
    #[serde(rename = "Count")]
    pub count: i8,
    #[serde(rename = "Damage")]
    pub damage: i16,
    pub id: i16,
}
