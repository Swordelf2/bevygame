use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Collider, Player};

pub fn collision(
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
