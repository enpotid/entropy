use crate::player::*;

use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform), With<Camera2d>>,
) {
    for ev in scroll_evr.read() {
        for (_camera, mut transform) in query.iter_mut() {
            let zoom_factor = 1.0 - ev.y * 0.1;
            transform.scale *= Vec3::splat(zoom_factor).clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            transform.scale = transform.scale.clamp(
                Vec3 {
                    x: 0.65,
                    y: 0.65,
                    z: 0.65,
                },
                Vec3 {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                },
            );
        }
    }
}

fn follow_player(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<(&Transform, &SpriteSizeState), (With<Player>, Without<Camera2d>)>,
) {
    if let Ok((player_transform, state)) = player_query.single() {
        if let Ok(mut cam_transform) = camera_query.single_mut() {
            cam_transform.translation.x = player_transform.translation.x;
            cam_transform.translation.y = player_transform.translation.y + state.height / 2.0;
        }
    }
}
