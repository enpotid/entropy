use bevy::prelude::*;

use crate::{map::*, sprite::SpriteSizeState};

pub fn collision_right(map: &Map, transform: &mut Transform, sprite_size: &SpriteSizeState) {
    for y in 1..sprite_size.height as usize {
        if let Some(tile) = is_solid_position(
            map,
            transform.translation.x + sprite_size.width,
            transform.translation.y + y as f32,
        ) {
            transform.translation.x = tile.left - sprite_size.width;
            return;
        }
    }
}
