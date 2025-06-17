pub mod movement;

use crate::sprite::{SpriteSizeState, SpriteState};
use movement::*;

use bevy::{prelude::*, sprite::Anchor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .insert_resource(PlayerStats::default());
    }
}

#[derive(Component)]
pub struct Player {
    pub jump: JumpKind,
}

#[derive(PartialEq)]
pub enum JumpKind {
    Up(f32),
    Down(f32),
    Stay,
}

#[derive(Resource)]
pub struct PlayerStats {
    pub speed: f32,
    pub jump_speed: f32,
    pub jump_height: f32,
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
            anchor: Anchor::BottomLeft,
            flip_x: false,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            jump: JumpKind::Stay,
        },
        SpriteState {
            is_falling: false,
            head_bumped: false,
        },
        SpriteSizeState {
            done: false,
            width: 800.0,
            height: 1600.0,
        },
    ));
}
