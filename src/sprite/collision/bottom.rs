use bevy::prelude::*;

use crate::{map::*, sprite::*};

pub fn collision_bottom(
    map: &Map,
    transform: &mut Transform,
    sprite_state: &mut SpriteState,
    sprite_size: &SpriteSizeState,
) {
    for x in 1..sprite_size.width as usize {
        if let Some(tile) = is_solid_position(
            map,
            transform.translation.x + x as f32,
            transform.translation.y - 1.0,
        ) {
            transform.translation.y = tile.top;
            sprite_state.is_falling = false;
            return;
        }
    }

    sprite_state.is_falling = true;
}
