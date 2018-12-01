use amethyst::ecs::{Component, NullStorage};

pub struct Ally {}

impl Default for Ally {
    fn default() -> Self {
        Self {}
    }
}

impl Component for Ally {
    type Storage = NullStorage<Self>;
}
