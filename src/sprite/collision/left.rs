use bevy::prelude::*;

use crate::{map::*, sprite::SpriteSizeState};

pub fn collision_left(map: &Map, transform: &mut Transform, sprite_size: &SpriteSizeState) {
    for y in 1..sprite_size.height as usize {
        if let Some(tile) = is_solid_position(
            map,
            transform.translation.x,
            transform.translation.y + y as f32,
        ) {
            transform.translation.x = tile.right;
            return;
        }
    }
}
