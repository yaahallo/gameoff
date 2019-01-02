use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Ally;

impl Component for Ally {
    type Storage = NullStorage<Self>;
}
