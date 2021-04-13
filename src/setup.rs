use bevy::prelude::*;

use crate::components::{Collider, Player};
use crate::config::CELL_SIZE;
use crate::resources::StartPos;

pub fn setup(
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
