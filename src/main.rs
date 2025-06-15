mod camera;
mod player;

use camera::*;
use player::*;

use bevy::{
    prelude::*,
    sprite::Anchor,
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
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.3, 0.7, 0.3),
            anchor: Anchor::TopCenter,
            custom_size: Some(Vec2::new(500.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
