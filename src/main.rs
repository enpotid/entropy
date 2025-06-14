use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, flip_sprite_on_input)
        .run();
}

#[derive(Component)]
struct Miku;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("miku.png"),
            custom_size: Some(Vec2 {
                x: 710.0,
                y: 1007.0,
            }),
            flip_x: false,
            ..Default::default()
        },
        Miku,
    ));
}

fn flip_sprite_on_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Transform), With<Miku>>,
) {
    for (mut sprite, mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.0;
            sprite.flip_x = true;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.0;
            sprite.flip_x = false;
        }
    }
}
