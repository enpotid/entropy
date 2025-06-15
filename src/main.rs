use bevy::{
    input::mouse::MouseWheel,
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
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, resize_sprite)
        .add_systems(Update, camera_zoom)
        .run();
}

#[derive(Component)]
struct Player {
    jumping: bool,
}

#[derive(Component)]
struct SpriteSizeState {
    done: bool,
    width: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("miku.png"),
            flip_x: false,
            ..Default::default()
        },
        Player { jumping: false },
        SpriteSizeState {
            done: false,
            width: 500.0,
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Player, &mut Transform), With<Player>>,
) {
    for (mut sprite, mut player, mut transform) in &mut query {
        let mut speed = 2.0;

        if keyboard_input.pressed(KeyCode::Space) {
            if transform.translation.y == 0.0 {
                player.jumping = true;
            }
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed *= 2.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed;
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += speed;
            sprite.flip_x = false;
        }

        if player.jumping {
            if transform.translation.y < 200.0 {
                transform.translation.y += ((200.02 - transform.translation.y).ln() + 4.0) * speed;

                if transform.translation.y > 200.0 {
                    transform.translation.y = 200.0;
                    player.jumping = false;
                }
            }
        } else {
            if transform.translation.y > 0.0 {
                transform.translation.y -= ((200.02 - transform.translation.y).ln() + 4.0) * speed;

                if transform.translation.y < 0.0 {
                    transform.translation.y = 0.0
                }
            }
        }
    }
}

fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform), With<Camera2d>>,
) {
    for ev in scroll_evr.read() {
        for (_camera, mut transform) in query.iter_mut() {
            let zoom_factor = 1.0 - ev.y * 0.1;
            transform.scale *= Vec3::splat(zoom_factor).clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        }
    }
}

fn resize_sprite(
    images: Res<Assets<Image>>,
    mut query: Query<(&mut Sprite, &mut SpriteSizeState)>,
) {
    for (mut sprite, mut state) in &mut query {
        if state.done {
            continue;
        }

        if let Some(image) = images.get(sprite.image.id()) {
            let size = image.size();
            let target_width = state.width;
            let ratio = size.y as f32 / size.x as f32;
            let target_height = target_width * ratio;

            sprite.custom_size = Some(Vec2::new(target_width, target_height));
            state.done = true;
        }
    }
}
