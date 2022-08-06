use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_player));
    }
}

fn setup_player(mut commands: Commands, game_assets: Res<GameAssets>, maps: Res<Assets<Map>>) {
    let map = maps.get(&game_assets.map).unwrap();
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: game_assets.player_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(
            TILE_WIDTH * map.player_spawn.0 as f32,
            TILE_HEIGHT * map.player_spawn.1 as f32,
            1.,
        )),
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        ..default()
    });
}
