use amethyst::ecs::{Component, DenseVecStorage};

pub struct Animation {
    pub total_frames: usize,
    pub max_count_till_next_frame: u32,
    pub frame_life_time_count: u32,
    pub current_frame: usize,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            total_frames: 0,
            max_count_till_next_frame: 0,
            frame_life_time_count: 0,
            current_frame: 0,
        }
    }
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}
