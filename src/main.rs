use bevy::prelude::*;
use core::f32::consts::PI;

use bevy::sprite::collide_aabb::collide;

#[derive(Default)]
struct Player {
    has_collided: bool,
}

struct Hazard;

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
        // Startup systems
        .add_startup_system(setup.system())
        // Systems
        .add_system(player_control.system())
        .add_system(collision_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load player texture
    let player_texture_handle: Handle<_> = asset_server.load("player.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 120.0)),
            ..Default::default()
        })
        .insert(Player::default());

    // Hazard squares
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.9).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 120.0)),
            ..Default::default()
        })
        .insert(Hazard);
}

fn player_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut().expect("No player entity found");

    // if has collided, reset position
    if player.has_collided {
        transform.translation = Vec3::new(0.0, -215.0, 0.0);
        player.has_collided = false;
    }

    const ROTATION_SPEED: f32 = 1.0;
    const SPEED: f32 = 300.0;

    let angle = transform.rotation.to_axis_angle().1 + time.delta_seconds() * ROTATION_SPEED;
    let angle = if angle >= 2.0 * PI {
        angle - 2.0 * PI
    } else {
        angle
    };

    transform.rotation = Quat::from_rotation_z(angle);

    if input.pressed(KeyCode::A) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * SPEED * angle.cos(),
            -time.delta_seconds() * SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(KeyCode::D) {
        transform.translation += Vec3::new(
            time.delta_seconds() * SPEED * angle.cos(),
            time.delta_seconds() * SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(KeyCode::W) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * SPEED * angle.sin(),
            time.delta_seconds() * SPEED * angle.cos(),
            0.0,
        );
    }
    if input.pressed(KeyCode::S) {
        transform.translation += Vec3::new(
            time.delta_seconds() * SPEED * angle.sin(),
            -time.delta_seconds() * SPEED * angle.cos(),
            0.0,
        );
    }

    /*
    let rotor = Quat::from_rotation_z(ROTATION_SPEED * time.delta_seconds);
    transform.rotate(rotor);
    */
}

fn collision_system(
    mut player_query: Query<(&Transform, &Sprite, &mut Player)>,
    hazard_query: Query<(&Transform, &Sprite), With<Hazard>>,
) {
    let (player_transform, player_sprite, mut player) =
        player_query.single_mut().expect("No player entity found");

    // Iterate over all hazards
    for (hazard_transform, hazard_sprite) in hazard_query.iter() {
        if collide(
            player_transform.translation,
            player_sprite.size,
            hazard_transform.translation,
            hazard_sprite.size,
        )
        .is_some()
        {
            player.has_collided = true;
            println!("Collided!");
        }
    }
}
