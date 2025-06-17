use crate::map::{self, *};

use bevy::{prelude::*, sprite::Anchor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, collision_player)
            .add_systems(Update, resize_sprite)
            .insert_resource(PlayerStats::default());
    }
}

#[derive(Component)]
pub struct Player {
    jump: JumpKind,
}

#[derive(PartialEq)]
enum JumpKind {
    Up(f32),
    Down(f32),
    Stay,
}

#[derive(Component)]
pub struct SpriteSizeState {
    pub done: bool,
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
struct PlayerStats {
    speed: f32,
    jump_speed: f32,
    jump_height: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            speed: 400.0,
            jump_speed: 100.0,
            jump_height: 1400.0,
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("miku.png"),
            anchor: Anchor::BottomCenter,
            flip_x: false,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            jump: JumpKind::Stay,
        },
        SpriteSizeState {
            done: false,
            width: 800.0,
            height: 1600.0,
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    stats: Res<PlayerStats>,
    map: Res<map::Map>,
    mut query: Query<(&mut Sprite, &mut Player, &mut Transform), With<Player>>,
) {
    for (mut sprite, mut player, mut transform) in &mut query {
        let mut speed = stats.speed * time.delta_secs();
        let jump_speed = stats.jump_speed * time.delta_secs();

        if keyboard_input.pressed(KeyCode::Space) {
            if let Some(tile) = map.tiles.get(&xy_to_position(
                transform.translation.x,
                transform.translation.y - 1.0,
            )) {
                if transform.translation.y % 1000.0 == 0.0 && is_solid(tile.kind) {
                    player.jump = JumpKind::Up(transform.translation.y);
                }
            }
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed *= 3.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed;
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += speed;
            sprite.flip_x = false;
        }

        if let JumpKind::Up(start_y) = player.jump {
            transform.translation.y +=
                ((start_y + stats.jump_height + 0.02 - transform.translation.y).ln() + 4.0)
                    * jump_speed;
        } else if let JumpKind::Down(start_y) = player.jump {
            transform.translation.y -=
                ((start_y + stats.jump_height + 0.02 - transform.translation.y).ln() + 4.0)
                    * jump_speed;
        }
    }
}

fn collision_player(
    stats: Res<PlayerStats>,
    map: Res<map::Map>,
    mut query: Query<(&mut Player, &mut Transform, &SpriteSizeState), With<Player>>,
) {
    for (mut player, mut transform, sprite_size) in &mut query {
        if let Some(tile) = map.tiles.get(&xy_to_position(
            transform.translation.x - sprite_size.width / 2.0 - 4.0,
            transform.translation.y + 1.0,
        )) {
            if is_solid(tile.kind) {
                transform.translation.x = tile.right + sprite_size.width / 2.0 + 4.0;
            } else if let Some(tile) = map.tiles.get(&xy_to_position(
                transform.translation.x - sprite_size.width / 2.0 - 4.0,
                transform.translation.y + sprite_size.width / 2.0,
            )) {
                if is_solid(tile.kind) {
                    transform.translation.x = tile.right + sprite_size.width / 2.0 + 4.0;
                } else if let Some(tile) = map.tiles.get(&xy_to_position(
                    transform.translation.x - sprite_size.width / 2.0 - 4.0,
                    transform.translation.y + sprite_size.width - 1.0,
                )) {
                    if is_solid(tile.kind) {
                        transform.translation.x = tile.right + sprite_size.width / 2.0 + 4.0;
                    }
                }
            }
        }

        if let Some(tile) = map.tiles.get(&xy_to_position(
            transform.translation.x + sprite_size.width / 2.0 + 4.0,
            transform.translation.y + 1.0,
        )) {
            if is_solid(tile.kind) {
                transform.translation.x = tile.left - sprite_size.width / 2.0 - 4.0;
            } else if let Some(tile) = map.tiles.get(&xy_to_position(
                transform.translation.x + sprite_size.width / 2.0 + 4.0,
                transform.translation.y + sprite_size.width / 2.0,
            )) {
                if is_solid(tile.kind) {
                    transform.translation.x = tile.left - sprite_size.width / 2.0 - 4.0;
                } else if let Some(tile) = map.tiles.get(&xy_to_position(
                    transform.translation.x + sprite_size.width / 2.0 + 4.0,
                    transform.translation.y + sprite_size.width - 1.0,
                )) {
                    if is_solid(tile.kind) {
                        transform.translation.x = tile.left - sprite_size.width / 2.0 - 4.0;
                    }
                }
            }
        }

        if let JumpKind::Up(start_y) = player.jump {
            if transform.translation.y >= start_y + stats.jump_height {
                player.jump = JumpKind::Down(start_y);
                transform.translation.y = start_y + stats.jump_height;
            }
        } else if let JumpKind::Down(_) = player.jump {
            if let Some(tile) = map.tiles.get(&xy_to_position(
                transform.translation.x + sprite_size.width / 2.0,
                transform.translation.y,
            )) {
                if is_solid(tile.kind) {
                    player.jump = JumpKind::Stay;
                    transform.translation.y = tile.top;
                } else if let Some(tile) = map.tiles.get(&xy_to_position(
                    transform.translation.x - sprite_size.width / 2.0,
                    transform.translation.y,
                )) {
                    if is_solid(tile.kind) {
                        player.jump = JumpKind::Stay;
                        transform.translation.y = tile.top;
                    }
                }
            }
        } else if player.jump == JumpKind::Stay {
            if let Some(tile) = map.tiles.get(&xy_to_position(
                transform.translation.x + sprite_size.width / 2.0,
                transform.translation.y - 1.0,
            )) {
                if !is_solid(tile.kind) {
                    if let Some(tile) = map.tiles.get(&xy_to_position(
                        transform.translation.x - sprite_size.width / 2.0,
                        transform.translation.y - 1.0,
                    )) {
                        if !is_solid(tile.kind) {
                            player.jump = JumpKind::Down(tile.bottom);
                        }
                    }
                }
            }
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
