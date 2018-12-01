use amethyst::ecs::{Component, NullStorage};

pub struct Enemy {}

impl Default for Enemy {
    fn default() -> Self {
        Self {}
    }
}

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}
