use bevy::prelude::*;

use bevygame::{setup, systems};

fn main() {
    App::build()
        // from learnbevy
        .insert_resource(WindowDescriptor {
            title: "bevygame".to_string(),
            width: 1024.0,
            height: 1024.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        // Default plaugins
        .add_plugins(DefaultPlugins)
        // Startup system
        .add_startup_system(setup::setup.system())
        // Systems
        .add_system(systems::animation.system())
        .add_system(systems::player_control.system())
        .add_system(systems::collision.system())
        .add_system(systems::score.system())
        .add_system(systems::camera_movement.system())
        .run();
}
