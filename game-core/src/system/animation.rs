use amethyst::{
    ecs::{Entities, Join, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::component::Animation;

pub struct Frame;

impl<'s> System<'s> for Frame {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Animation>,
        Entities<'s>,
    );

    fn run(&mut self, (mut sprite_render, mut animation, entities): Self::SystemData) {
        for (_, animation, sprite_render) in (&entities, &mut animation, &mut sprite_render).join()
        {
            frame_update(animation, sprite_render);
        }
    }
}

fn frame_update(animation: &mut Animation, sprite_render: &mut SpriteRender) {
    if animation.frame_life_time_count > 0 {
        animation.frame_life_time_count = animation.frame_life_time_count - 1;
    } else {
        animation.frame_life_time_count = animation.max_count_till_next_frame;
        animation.current_frame = (animation.current_frame + 1) % animation.total_frames;
    }

    sprite_render.sprite_number = animation.current_frame;
}
