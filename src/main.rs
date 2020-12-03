use bevy::prelude::*;


// Marker component
struct Player;

fn main() {
    App::build()
        // from learnbevy
        .add_resource(WindowDescriptor {
            title: "bevygame".to_string(),
            width: 1024,
            height: 1024,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        // Default plaugins
        .add_plugins(DefaultPlugins)
        // Startup systems
        .add_startup_system(setup.system())
        // Systems
        .add_system(player_control.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load player texture
    let player_texture_handle: Handle<_> = asset_server.load("player.png");

    commands
        .spawn(Camera2dComponents::default())
        // player
        .spawn(SpriteComponents {
            material: materials.add(player_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 120.0)),
            ..Default::default()
        })
        .with(Player);
}

fn player_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    _player: &Player,
    mut transform: Mut<Transform>,
) {

    const ROTATION_SPEED: f32 = 1.0;
    const SPEED: f32 = 300.0;

    let angle = transform.rotation.to_axis_angle().1 + time.delta_seconds * ROTATION_SPEED;
    let angle = if angle >= 2.0 * PI 
        { angle - 2.0 * PI  } 
        else { angle };

    transform.rotation = Quat::from_rotation_z(angle);

    
    if input.pressed(KeyCode::A) {
        transform.translation += Vec3::new(-time.delta_seconds * SPEED * angle.cos(), -time.delta_seconds * SPEED * angle.sin(), 0.0);
    }
    if input.pressed(KeyCode::D) {
        transform.translation += Vec3::new(time.delta_seconds * SPEED * angle.cos(), time.delta_seconds * SPEED * angle.sin(), 0.0);
    }
    if input.pressed(KeyCode::W) {
        transform.translation += Vec3::new(-time.delta_seconds * SPEED * angle.sin(), time.delta_seconds * SPEED * angle.cos(), 0.0);
    }
    if input.pressed(KeyCode::S) {
        transform.translation += Vec3::new(time.delta_seconds * SPEED * angle.sin(), -time.delta_seconds * SPEED * angle.cos(), 0.0);
    }





    /*
    let rotor = Quat::from_rotation_z(ROTATION_SPEED * time.delta_seconds);
    transform.rotate(rotor);
    */

}
