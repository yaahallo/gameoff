use amethyst::core::cgmath::InnerSpace;
use amethyst::core::cgmath::Vector2;
use amethyst::renderer::Camera;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use crate::component::Player;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, cameras, mut transforms): Self::SystemData) {
        let mut player_translation = Vector2 {
            x: 0.0 as f32,
            y: 0.0 as f32,
        };

        for (_, transform) in (&players, &mut transforms).join() {
            player_translation = transform.translation.truncate() - Vector2 { x: 256.0, y: 256.0 };
        }

        for (_, transform) in (&cameras, &mut transforms).join() {
            let camera_translation = transform.translation.truncate();
            let player_direction = player_translation - camera_translation;
            let camera_safe_edge =
                player_direction.normalize_to((transform.scale.truncate() / 4 as f32).magnitude());

            if player_direction.magnitude2() > camera_safe_edge.magnitude2() {
                let camera_shift = player_direction - camera_safe_edge;
                transform.translation.x += camera_shift.x;
                transform.translation.y += camera_shift.y;
            }
        }
    }
}
