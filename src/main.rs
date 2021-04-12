use bevy::prelude::*;
use core::f32::consts::PI;

use bevy::render::camera::Camera;
use bevy::sprite::collide_aabb::collide;

// Cell size in pixels
const CELL_SIZE: f32 = 32.0;
const MOVE_SPEED: f32 = CELL_SIZE * 3.0;
const ROTATION_SPEED: f32 = 1.0;

#[derive(Default)]
struct Player {
    has_collided: Option<Collider>,
}

#[derive(Clone, Copy)]
enum Collider {
    Hazard,
    Princess,
}

// StartPos resource
struct StartPos {
    x: f32,
    y: f32,
}

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
        .add_system(score_system.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load player and princess textures
    let player_material_handle = materials.add(asset_server.load("player.png").into());
    let princess_material_handle = materials.add(asset_server.load("princess.png").into());

    // Load map image
    let map_img = image::open("assets/map.png")
        .expect("File map.png not found")
        .into_rgb8();

    // Iterate over the pixels of the image and spawn corresponding cells (player and hazards)
    for (x, mut y, &pixel) in map_img.enumerate_pixels() {
        // Invert the y axis
        y = map_img.height() - y - 1;
        // Calculate cell position
        let cell_pos_x = x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let cell_pos_y = y as f32 * CELL_SIZE + CELL_SIZE / 2.0;

        match pixel.0 {
            // Red = Princess (finish cell)
            [255, 0, 0] => {
                // Spawn princess
                commands
                    .spawn_bundle(SpriteBundle {
                        material: princess_material_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            cell_pos_x, cell_pos_y, 0.1,
                        )),
                        sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    })
                    .insert(Collider::Princess);
            }
            // Green = Start cell
            [0, 255, 0] => {
                // Spawn player
                commands
                    .spawn_bundle(SpriteBundle {
                        material: player_material_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            cell_pos_x, cell_pos_y, 0.2,
                        )),
                        sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    })
                    .insert(Player::default());
                // Store the start position as a resource
                commands.insert_resource(StartPos {
                    x: cell_pos_x,
                    y: cell_pos_y,
                });
            }
            // Blue = Hazard cell
            [0, 0, 255] => {
                // Spawn a hazard cell
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(Color::rgb(0.0, 0.0, 0.9).into()),
                        transform: Transform::from_translation(Vec3::new(
                            cell_pos_x, cell_pos_y, 0.1,
                        )),
                        sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    })
                    .insert(Collider::Hazard);
            }
            _ => {}
        }
    }

    // Spawn camera
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);
}

fn player_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut().expect("No player entity found");

    let angle = transform.rotation.to_axis_angle().1 + time.delta_seconds() * ROTATION_SPEED;
    let angle = if angle >= 2.0 * PI {
        angle - 2.0 * PI
    } else {
        angle
    };

    transform.rotation = Quat::from_rotation_z(angle);

    if input.pressed(KeyCode::A) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * MOVE_SPEED * angle.cos(),
            -time.delta_seconds() * MOVE_SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(KeyCode::D) {
        transform.translation += Vec3::new(
            time.delta_seconds() * MOVE_SPEED * angle.cos(),
            time.delta_seconds() * MOVE_SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(KeyCode::W) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * MOVE_SPEED * angle.sin(),
            time.delta_seconds() * MOVE_SPEED * angle.cos(),
            0.0,
        );
    }
    if input.pressed(KeyCode::S) {
        transform.translation += Vec3::new(
            time.delta_seconds() * MOVE_SPEED * angle.sin(),
            -time.delta_seconds() * MOVE_SPEED * angle.cos(),
            0.0,
        );
    }

    /*
    let rotor = Quat::from_rotation_z(ROTATION_SPEED * time.delta_seconds);
    transform.rotate(rotor);
    */
}

fn camera_movement(
    mut q: QuerySet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    // Retrieve player's position and size
    let player_transform = q.q1().single().expect("Player entity not found");
    let player_pos = *player_transform.translation;

    let mut camera_transform = q.q0_mut().single_mut().expect("Camera entity not found");
    camera_transform.translation.x = player_pos.x;
    camera_transform.translation.y = player_pos.y;
}

fn score_system(start_pos: Res<StartPos>, mut player_query: Query<(&mut Transform, &mut Player)>) {
    let (mut transform, mut player) = player_query.single_mut().expect("Player entity not found");
    // if has collided, reset position
    match player.has_collided {
        Some(Collider::Hazard) => {
            transform.translation.x = start_pos.x;
            transform.translation.y = start_pos.y;
            player.has_collided = None;
        }
        Some(Collider::Princess) => {
            println!("YOU WON");
            transform.translation.x = start_pos.x;
            transform.translation.y = start_pos.y;
            player.has_collided = None;
        }
        None => {}
    }
}

fn collision_system(
    mut player_query: Query<(&Transform, &Sprite, &mut Player)>,
    collider_query: Query<(&Transform, &Sprite, &Collider)>,
) {
    let (player_transform, player_sprite, mut player) =
        player_query.single_mut().expect("No player entity found");

    // Iterate over all hazards
    for (collider_transform, collider_sprite, collider) in collider_query.iter() {
        if collide(
            player_transform.translation,
            player_sprite.size,
            collider_transform.translation,
            collider_sprite.size,
        )
        .is_some()
        {
            player.has_collided = Some(*collider);
            println!("Collided!");
        }
    }
}
