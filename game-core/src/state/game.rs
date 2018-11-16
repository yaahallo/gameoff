use amethyst::prelude::*;
use component::Player;
use crate::load;

pub struct Game;

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.add_resource(load::LoadedTextures::default());

        let circle_sprite_sheet_handle =
            load::sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");

        crate::map::load_map_sprites(world);
        let parent = Player::new(world, &circle_sprite_sheet_handle);
        init::camera(world, parent);
    }
}

mod init {
    use amethyst::utils::ortho_camera::CameraNormalizeMode;
    use amethyst::utils::ortho_camera::CameraOrtho;
    use amethyst::{
        core::{Parent, Transform},
        ecs::Entity,
        prelude::*,
        renderer::Camera,
    };

    pub fn camera(world: &mut World, parent: Entity) {
        let mut transform = Transform::default();
        transform.translation.z = 2.0;
        transform.translation.x = -256.0;
        transform.translation.y = -256.0;
        transform.scale.x = 512.0;
        transform.scale.y = 512.0;

        world.register::<CameraOrtho>();

        world
            .create_entity()
            .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
            .with(Camera::standard_2d())
            .with(Parent { entity: parent })
            .with(transform)
            .build();
    }
}
