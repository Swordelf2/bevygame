use bevy::prelude::*;

use crate::components::Animation;

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut timer, mut sprite, animation) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            println!("animation.len = {}, sprite.index = {}", animation.len, sprite.index);
            if sprite.index + 1 >= animation.len {
                sprite.index = 0;
            } else {
                sprite.index = sprite.index + 1;
            }
        }
    }
}
