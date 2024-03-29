use bevy::prelude::*;

use crate::components::{Collider, Player};
use crate::config::{mapcolors, paths, CELL_SIZE};
use crate::resources::StartPos;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load player texture
    let player_material_handle = materials.add(asset_server.load(paths::PLAYER_SPRITE).into());

    // Load princess texture
    let princess_material_handle = materials.add(asset_server.load(paths::PRINCESS_SPRITE).into());

    /*
    // Load player texture atlas
    let (player_atlas_len, player_atlas_handle) = {
        let texture_handle = asset_server.load("textures/player/player.png");
        let atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(720.0, 490.0), 18, 1);
        // let something_handle = asset_server.get_handle("path to something within the atlas");
        // let something_index = atlas.get_texture_index(&something_handle).unwrap();
        (atlas.textures.len(), atlas_assets.add(atlas))
    };
    */

    // Load map image
    let map_img = image::open(paths::MAP)
        .unwrap()
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
            mapcolors::PRINCESS => {
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
            mapcolors::PLAYER => {
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
                    .insert(Player::default())
                    // Spawn child camera
                    .with_children(|parent| {
                        let mut camera = OrthographicCameraBundle::new_2d();
                        // TODO camera bug: hazard are drawn with this line ??
                        // absolute z should be = `far` - `eps`
                        camera.transform.translation.z = 1000.0 - 0.5 - 0.2;
                        parent.spawn_bundle(camera);
                    });
                // Store the start position as a resource
                commands.insert_resource(StartPos {
                    x: cell_pos_x,
                    y: cell_pos_y,
                });
            }
            // Blue = Hazard cell
            mapcolors::HAZARD => {
                // Spawn a hazard cell
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(Color::rgb(0.0, 0.0, 0.95).into()),
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
}
