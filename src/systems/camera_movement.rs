use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::components::Player;

pub fn camera_movement(
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
