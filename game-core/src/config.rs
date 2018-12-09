use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Ally {
    pub max_hp: u32,
    pub follow_distance: f32,
    pub max_distance: f32,
    pub min_distance: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Player {
    pub max_hp: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Enemy {
    pub max_hp: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameoffConfig {
    pub ally: Ally,
    pub player: Player,
    pub enemy: Enemy,
    pub speed: f32,
}
