use bevy::prelude::*;

use crate::components::{Collider, Player};
use crate::resources::StartPos;

pub fn score(start_pos: Res<StartPos>, mut player_query: Query<(&mut Transform, &mut Player)>) {
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
