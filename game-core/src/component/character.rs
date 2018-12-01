use amethyst::ecs::{Component, HashMapStorage};

pub struct Character {
    pub hp: u32,
    pub max_hp: u32,
}

impl Default for Character {
    fn default() -> Character {
        Character { hp: 0, max_hp: 0 }
    }
}

impl Component for Character {
    type Storage = HashMapStorage<Self>;
}
