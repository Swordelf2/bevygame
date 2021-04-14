use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::components::{Collider, Player};

pub struct GameDebug;

impl Plugin for GameDebug {
    fn build(&self, app: &mut AppBuilder) {
        // TODO Add stage "debug" after update. All these systems should go there
        app.add_system(player_and_camera_positions.system());
    }
}

fn player_and_camera_positions(
    q_player: Query<(&Transform, &GlobalTransform), With<Player>>,
    q_camera: Query<(&Transform, &GlobalTransform), With<Camera>>,
    q_hazard: Query<(&Transform, &GlobalTransform), With<Collider>>,
) {
    let (player_transform, player_global_transform) = q_player.single().expect("Player not found");
    let (camera_transform, camera_global_transform) = q_camera.single().expect("Camera not found");
    let (hazard_transform, hazard_global_transform) = q_hazard.iter().next().expect("Hazard not found");
    println!(
        "player_transform = {:#?}\nplayer_global_transform = {:#?}\n\
            camera_transform = {:#?}\ncamera_global_transform = {:#?}\n\
            hazard_transform = {:#?}\nhazard_global_transform = {:#?}",
        player_transform,
        player_global_transform,
        camera_transform,
        camera_global_transform,
        hazard_transform,
        hazard_global_transform
    );
}
