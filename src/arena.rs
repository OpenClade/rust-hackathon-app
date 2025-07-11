use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct ArenaState {
    #[serde(rename = "turnNo")]
    pub turn_no: u32,
    #[serde(rename = "nextTurnIn")]
    pub next_turn_in: f32,
    pub ants: Vec<Ant>,
    pub enemies: Vec<EnemyAnt>,
    pub map: Vec<MapCell>,
    pub food: Vec<Food>,
    pub home: Vec<HexCoord>,
    pub score: u32,
    pub spot: HexCoord,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ant {
    pub id: String,
    #[serde(rename = "type")]
    pub ant_type: u8,
    pub q: i32,
    pub r: i32,
    pub health: u32,
    pub food: AntFood,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AntFood {
    #[serde(rename = "type")]
    pub food_type: u8,
    pub amount: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyAnt {
    #[serde(rename = "type")]
    pub ant_type: u8,
    pub q: i32,
    pub r: i32,
    pub health: u32,
    pub attack: u32,
    pub food: Option<AntFood>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapCell {
    pub q: i32,
    pub r: i32,
    pub cost: u8,
    #[serde(rename = "type")]
    pub cell_type: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Food {
    pub q: i32,
    pub r: i32,
    #[serde(rename = "type")]
    pub food_type: u8,
    pub amount: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

pub async fn get_arena_info() -> Result<ArenaState, reqwest::Error> {
    let client = Client::new();
    let url = "https://games-test.datsteam.dev/api/arena";
    let token = env::var("API_TOKEN").expect("API_TOKEN not set");

    let resp = client
        .get(url)
        .header("accept", "application/json")
        .header("X-Auth-Token", token)
        .send()
        .await?
        .json::<ArenaState>()
        .await?;

    Ok(resp)
}
