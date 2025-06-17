use crate::sprite::SpriteSizeState;

use bevy::{prelude::*, sprite::Anchor};
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map).insert_resource(Map {
            tiles: HashMap::new(),
        });
    }
}

#[derive(Component, Clone, Copy)]
pub struct Tile {
    pub kind: TileKind,
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Clone, Copy)]
pub enum TileKind {
    Plains,
    Sky,
}

#[derive(Resource)]
pub struct Map {
    pub tiles: HashMap<Position, Tile>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Position(i64, i64);

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut map: ResMut<Map>) {
    let mut m = [
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 1, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 1, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
    ];
    m.reverse();

    for i in 0..m.len() as i64 {
        for ii in 0..m[0].len() as i64 {
            let tile = Tile {
                kind: if m[i as usize][ii as usize] == 0 {
                    TileKind::Sky
                } else {
                    TileKind::Plains
                },
                top: 1000.0 * (i - m.len() as i64 / 2) as f32 + 1000.0,
                bottom: 1000.0 * (i - m.len() as i64 / 2) as f32,
                left: 1000.0 * (ii - m[0].len() as i64 / 2) as f32,
                right: 1000.0 * (ii - m[0].len() as i64 / 2) as f32 + 1000.0,
            };

            commands.spawn((
                Sprite {
                    image: if m[i as usize][ii as usize] == 0 {
                        asset_server.load("map/sky.png")
                    } else {
                        asset_server.load("map/plains.png")
                    },
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                Transform::from_xyz(
                    1000.0 * (ii - m[0].len() as i64 / 2) as f32,
                    1000.0 * (i - m.len() as i64 / 2) as f32,
                    0.0,
                ),
                tile,
                SpriteSizeState {
                    done: false,
                    width: 1000.0,
                    height: 1000.0,
                },
            ));

            map.tiles.insert(
                Position(
                    (ii - m[0].len() as i64 / 2) as i64,
                    (i - m.len() as i64 / 2) as i64,
                ),
                tile,
            );
        }
    }
}

pub fn xy_to_position(x: f32, y: f32) -> Position {
    Position(
        if x >= 0.0 || x % 1000.0 == 0.0 {
            x as i64 / 1000
        } else {
            x as i64 / 1000 - 1
        },
        if y >= 0.0 || y % 1000.0 == 0.0 {
            y as i64 / 1000
        } else {
            y as i64 / 1000 - 1
        },
    )
}

pub fn is_solid_position(map: &Map, x: f32, y: f32) -> Option<Tile> {
    if let Some(tile) = map.tiles.get(&xy_to_position(x, y)) {
        if is_solid(tile.kind) {
            Some(*tile)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn is_not_solid_position(map: &Map, x: f32, y: f32) -> Option<Tile> {
    if let Some(tile) = map.tiles.get(&xy_to_position(x, y)) {
        if !is_solid(tile.kind) {
            Some(*tile)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn is_solid(kind: TileKind) -> bool {
    match kind {
        TileKind::Plains => true,
        TileKind::Sky => false,
    }
}
