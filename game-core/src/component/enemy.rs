use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}
