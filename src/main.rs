use bevy::prelude::*;

use bevygame::{setup, systems};

fn main() {
    let mut app = App::build();
        // from learnbevy
    app.insert_resource(WindowDescriptor {
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
        //.add_system(systems::camera_movement.system())
        .add_system(systems::collision.system())
        .add_system(systems::score.system());

    #[cfg(feature = "debug")]
    app.add_plugin(bevygame::debug::GameDebug);

    app.run();
}
