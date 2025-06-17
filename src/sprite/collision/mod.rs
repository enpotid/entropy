pub mod bottom;
pub mod left;
pub mod right;
pub mod top;

use crate::{map::*, sprite::*};
use bottom::*;
use left::*;
use right::*;
use top::*;

use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sprite_collision);
    }
}

fn sprite_collision(
    map: Res<Map>,
    mut query: Query<(&mut Transform, &mut SpriteState, &SpriteSizeState), With<Sprite>>,
) {
    for (mut transform, mut sprite_state, sprite_size) in &mut query {
        collision_left(&map, &mut transform, sprite_size);
        collision_right(&map, &mut transform, sprite_size);
        collision_top(&map, &mut transform, sprite_size);
        collision_bottom(&map, &mut transform, &mut sprite_state, sprite_size);
    }
}
