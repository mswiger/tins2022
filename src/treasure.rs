use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// Make it a little harder to grab treasure
const TREASURE_SHRINKAGE: f32 = 4.;

#[derive(Component)]
pub struct Treasure;

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_treasure));
    }
}

fn setup_treasure(mut commands: Commands, game_assets: Res<GameAssets>, maps: Res<Assets<Map>>) {
    let map = maps.get(&game_assets.map).unwrap();

    for treasure in map.treasures.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_assets.coin_image.clone(),
                transform: Transform::from_translation(Vec3::new(
                    TILE_WIDTH * treasure.0 as f32,
                    TILE_HEIGHT * treasure.1 as f32,
                    1.,
                )),
                ..default()
            })
            .insert(Treasure)
            .insert(Collider::cuboid(
                TILE_WIDTH / 2. - TREASURE_SHRINKAGE,
                TILE_HEIGHT / 2. - TREASURE_SHRINKAGE,
            ))
            .insert(Sensor);
    }
}
