use crate::{map::*, sprite::player::*};

use bevy::prelude::*;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    stats: Res<PlayerStats>,
    map: Res<Map>,
    mut query: Query<
        (
            &mut Sprite,
            &mut Player,
            &mut Transform,
            &SpriteSizeState,
            &SpriteState,
        ),
        With<Player>,
    >,
) {
    for (mut sprite, mut player, mut transform, sprite_size, sprite_state) in &mut query {
        let mut speed = stats.speed * time.delta_secs();
        let jump_speed = stats.jump_speed * time.delta_secs();

        if keyboard_input.pressed(KeyCode::Space) {
            if !sprite_state.is_falling {
                player.jump = JumpKind::Up(transform.translation.y);
            }
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed *= 3.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            if let Some(_) = is_not_solid_position(
                &map,
                transform.translation.x - speed,
                transform.translation.y + 1.0,
            ) {
                transform.translation.x -= speed;
            }

            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            if let Some(_) = is_not_solid_position(
                &map,
                transform.translation.x + sprite_size.width + speed,
                transform.translation.y + 1.0,
            ) {
                transform.translation.x += speed;
            }

            sprite.flip_x = false;
        }

        if let JumpKind::Up(start_y) = player.jump {
            if sprite_state.head_bumped {
                player.jump = JumpKind::Down(start_y);
            } else {
                transform.translation.y +=
                    ((start_y + stats.jump_height + 0.02 - transform.translation.y).ln() + 4.0)
                        * jump_speed;

                if start_y + stats.jump_height <= transform.translation.y {
                    transform.translation.y = start_y + stats.jump_height;
                    player.jump = JumpKind::Down(start_y);
                }
            }
        } else if let JumpKind::Down(start_y) = player.jump {
            if !sprite_state.is_falling {
                player.jump = JumpKind::Stay;
            } else {
                transform.translation.y -=
                    ((start_y + stats.jump_height + 0.02 - transform.translation.y).ln() + 4.0)
                        * jump_speed;

                if transform.translation.y < start_y {
                    transform.translation.y = start_y;
                    player.jump = JumpKind::Stay;
                }
            }
        } else if sprite_state.is_falling {
            if let Some(tile) =
                is_not_solid_position(&map, transform.translation.x, transform.translation.y - 1.0)
            {
                player.jump = JumpKind::Down(tile.bottom);
            }
        }
    }
}
