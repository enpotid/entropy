use bevy::prelude::*;

use crate::{map::*, sprite::*};

pub fn collision_top(
    map: &Map,
    transform: &mut Transform,
    sprite_state: &mut SpriteState,
    sprite_size: &SpriteSizeState,
) {
    for x in 1..sprite_size.width as usize {
        if let Some(tile) = is_solid_position(
            map,
            transform.translation.x + x as f32,
            transform.translation.y + sprite_size.height,
        ) {
            transform.translation.y = tile.bottom - sprite_size.height;
            sprite_state.head_bumped = true;
            return;
        }
    }

    sprite_state.head_bumped = false;
}
