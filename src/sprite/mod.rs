pub mod collision;
pub mod player;

use collision::*;
use player::*;

use bevy::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(CollisionPlugin)
            .add_systems(Update, resize_sprite);
    }
}

#[derive(Component)]
pub struct SpriteSizeState {
    pub done: bool,
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct SpriteState {
    pub is_falling: bool,
    pub head_bumped: bool,
}

pub fn resize_sprite(
    images: Res<Assets<Image>>,
    mut query: Query<(&mut Sprite, &mut SpriteSizeState)>,
) {
    for (mut sprite, mut state) in &mut query {
        if state.done {
            continue;
        }

        if let Some(image) = images.get(sprite.image.id()) {
            let size = image.size();
            let mut target_width = state.width;
            let mut ratio = size.y as f32 / size.x as f32;
            let mut target_height = target_width * ratio;

            if target_height > state.height {
                target_height = state.height;
                ratio = size.x as f32 / size.y as f32;
                target_width = target_height * ratio;
            }

            sprite.custom_size = Some(Vec2::new(target_width, target_height));
            state.done = true;
        }
    }
}
