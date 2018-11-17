use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy {
    pub hp: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self { hp: 120 }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
