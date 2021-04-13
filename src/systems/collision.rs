use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Collider, Player};
use crate::config::CELL_SIZE;


pub fn collision(
    mut player_query: Query<(&Transform, &mut Player)>,
    collider_query: Query<(&Transform, &Sprite, &Collider)>,
) {
    let (player_transform, mut player) =
        player_query.single_mut().expect("No player entity found");

    // Iterate over all hazards
    for (collider_transform, collider_sprite, collider) in collider_query.iter() {
        if collide(
            player_transform.translation,
            Vec2::new(CELL_SIZE, CELL_SIZE),
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
