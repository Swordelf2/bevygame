use bevy::prelude::*;

use core::f32::consts::PI;

use crate::components::Player;
use crate::config::{MOVE_SPEED, ROTATION_SPEED};

pub fn player_control(
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
