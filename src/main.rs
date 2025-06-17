mod camera;
mod map;
mod sprite;

use camera::*;
use map::*;
use sprite::*;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Entropy".to_string(),
                mode: WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Current,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SpritePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .run();
}
