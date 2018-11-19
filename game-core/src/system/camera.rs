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
        let mut player_x: f32 = 0.0;
        let mut player_y: f32 = 0.0;

        // let mut camera_x: f32 = 0.0;
        // let mut camera_y: f32 = 0.0;

        // let mut camera_safe_x: f32 = 0.0;
        // let mut camera_safe_y: f32 = 0.0;

        for (_, transform) in (&players, &mut transforms).join() {
            player_x = transform.translation.x - 256.0 as f32;// * 5.0;
            player_y = transform.translation.y - 256.0 as f32;// * 5.0;
        }

        for (_, transform) in (&cameras, &mut transforms).join() {
            let camera_x = transform.translation.x as f32;// * 5.0;
            let camera_y = transform.translation.y as f32;// * 5.0;
            let camera_safe_x = transform.scale.x/4 as f32;
            let camera_safe_y = transform.scale.y/4 as f32;

            if (player_x - camera_x).abs() > camera_safe_x || (player_y - camera_y).abs() > camera_safe_y {
                println!("player escaped camera safe zone");
                transform.translation.x = player_x;
                transform.translation.y = player_y;
            }   

        }

    }
}