use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::component::Player;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Option<Read<'s, crate::map::PassableTiles>>,
    );

    fn run(&mut self, (players, mut transforms, input, passable): Self::SystemData) {
        if let Some(passable) = passable {
            let x_move = input.axis_value("entity_x").unwrap();
            let y_move = input.axis_value("entity_y").unwrap();

            for (_, transform) in (&players, &mut transforms).join() {
                let goal_x = transform.translation().x + x_move as f32 * 5.0;
                let goal_y = transform.translation().y + y_move as f32 * 5.0;

                let tile_y = (goal_y as u32 / 32) as usize;
                let tile_x = (goal_x as u32 / 32) as usize;

                if passable.tile_matrix[tile_y][tile_x] {
                    transform.set_x(goal_x).set_y(goal_y);
                }
            }
        }
    }
}
