use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ally {
    pub max_hp: u32,
    pub follow_distance: f32,
    pub max_distance: f32,
    pub min_distance: f32,
}

impl Default for Ally {
    fn default() -> Self {
        Ally {
            max_hp: 0,
            follow_distance: 0.0,
            max_distance: 0.0,
            min_distance: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub max_hp: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self { max_hp: 0 }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Enemy {
    pub max_hp: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self { max_hp: 0 }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameoffConfig {
    pub ally: Ally,
    pub player: Player,
    pub enemy: Enemy,
    pub speed: f32,
}

impl Default for GameoffConfig {
    fn default() -> Self {
        GameoffConfig {
            ally: Ally::default(),
            player: Player::default(),
            enemy: Enemy::default(),
            speed: 0.0,
        }
    }
}
