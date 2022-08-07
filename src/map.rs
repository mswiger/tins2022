use super::app::AppState;
use super::assets::GameAssets;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_rapier2d::prelude::*;
use std::str;

pub const MAP_WIDTH: usize = 36;
pub const MAP_HEIGHT: usize = 20;
pub const TILE_WIDTH: f32 = 16.;
pub const TILE_HEIGHT: f32 = 16.;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, TypeUuid)]
#[uuid = "e44e9629-7b52-41aa-94de-0a3bc1146b1e"]
pub struct Map {
    tiles: [[Tile; MAP_WIDTH]; MAP_HEIGHT],
    pub player_spawn: (u32, u32),
}

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut map = Map {
                tiles: [[Tile::Empty; MAP_WIDTH]; MAP_HEIGHT],
                player_spawn: (0, 0),
            };
            let map_str = str::from_utf8(bytes).unwrap().trim();
            let lines = map_str
                .lines()
                .collect::<Vec<&str>>()
                .into_iter()
                .rev()
                .collect::<Vec<&str>>();

            for i in 0..MAP_HEIGHT {
                let cells: Vec<&str> = lines[i].trim().split_whitespace().collect();
                for j in 0..MAP_WIDTH {
                    let tile_char = cells[j].chars().nth(0).unwrap();
                    map.tiles[i][j] = match tile_char {
                        '0' => Tile::Empty,
                        '1' => Tile::Wall,
                        'P' => {
                            map.player_spawn = (j as u32, i as u32);
                            Tile::Empty
                        }
                        _ => Tile::Empty,
                    }
                }
            }
            load_context.set_default_asset(LoadedAsset::new(map));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Map>()
            .init_asset_loader::<MapLoader>()
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_map)
                    .with_system(setup_boundaries)
                    .with_system(setup_music),
            );
    }
}

fn setup_map(mut commands: Commands, maps: Res<Assets<Map>>, game_assets: Res<GameAssets>) {
    let map = maps.get(&game_assets.map).unwrap();

    for i in 0..MAP_HEIGHT {
        for j in 0..MAP_WIDTH {
            let tile = map.tiles[i][j];
            let tile_index = match tile {
                Tile::Wall => 0,
                _ => 1,
            };
            let mut entity = commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_assets.tile_set_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(
                    TILE_WIDTH * j as f32,
                    TILE_HEIGHT * i as f32,
                    0.,
                )),
                sprite: TextureAtlasSprite {
                    index: tile_index,
                    ..default()
                },
                ..default()
            });

            if tile == Tile::Wall {
                // Add a little overlap between colliders to prevent player from getting stuck
                // between tiles.
                entity.insert(Collider::cuboid(8.1, 8.1));
            }
        }
    }
}

fn setup_boundaries(mut commands: Commands) {
    // Floor boundary
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform {
                translation: Vec3::new(MAP_WIDTH as f32 * TILE_WIDTH / 2., -TILE_HEIGHT / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(MAP_WIDTH as f32 * TILE_WIDTH / 2., 0.));

    // Left wall boundary
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform {
                translation: Vec3::new(-TILE_WIDTH / 2., MAP_HEIGHT as f32 * TILE_HEIGHT / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(0., MAP_HEIGHT as f32 * TILE_HEIGHT / 2.));

    // Right wall boundary
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform {
                translation: Vec3::new(
                    MAP_WIDTH as f32 * TILE_WIDTH - TILE_WIDTH / 2.,
                    MAP_HEIGHT as f32 * TILE_HEIGHT / 2.,
                    0.,
                ),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(0., MAP_HEIGHT as f32 * TILE_HEIGHT / 2.));
}

fn setup_music(game_assets: Res<GameAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        game_assets.bgm.clone(),
        PlaybackSettings::LOOP.with_volume(0.75),
    );
}
