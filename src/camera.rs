use crate::player::*;

use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_and_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn zoom_and_follow(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform), With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    windows: Query<&Window>,
) {
    for (_camera, mut transform) in query.iter_mut() {
        for ev in scroll_evr.read() {
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

        if let Ok(player_transform) = player_query.single() {
            if let Ok(window) = windows.single() {
                transform.translation.x = player_transform.translation.x;
                transform.translation.y = (window.height() / 2.0
                    - if window.height() / 2.0 > 200.0 {
                        100.0
                    } else {
                        0.0
                    })
                    * transform.scale.y
                    + player_transform.translation.y;
            }
        }
    }
}
