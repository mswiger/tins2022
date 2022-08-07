use super::animation::{Animation, AnimationData, AnimationState};
use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use benimator::Frame;
use bevy::prelude::*;
use std::time::Duration;

struct PlayerAnimations {
    idle: Handle<AnimationData>,
    walk: Handle<AnimationData>,
    jump: Handle<AnimationData>,
    swim: Handle<AnimationData>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_player));
    }
}

fn setup_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    maps: Res<Assets<Map>>,
    mut animations: ResMut<Assets<AnimationData>>,
) {
    let idle = AnimationData(benimator::Animation::from_range(
        0..=0,
        Duration::from_millis(1),
    ));
    let idle_handle = animations.add(idle);

    let walk = AnimationData(benimator::Animation::from_frames(vec![
        Frame::new(1, Duration::from_millis(250)),
        Frame::new(2, Duration::from_millis(250)),
        Frame::new(3, Duration::from_millis(250)),
        Frame::new(2, Duration::from_millis(250)),
    ]));
    let walk_handle = animations.add(walk);

    let swim = AnimationData(benimator::Animation::from_frames(vec![
        Frame::new(6, Duration::from_millis(250)),
        Frame::new(7, Duration::from_millis(250)),
    ]));
    let swim_handle = animations.add(swim);

    let jump = AnimationData(benimator::Animation::from_frames(vec![
        Frame::new(4, Duration::from_millis(250)),
        Frame::new(5, Duration::from_millis(250)),
    ]));
    let jump_handle = animations.add(jump);

    commands.insert_resource(PlayerAnimations {
        idle: idle_handle.clone(),
        walk: walk_handle.clone(),
        jump: jump_handle.clone(),
        swim: swim_handle.clone(),
    });

    let map = maps.get(&game_assets.map).unwrap();
    commands
        .spawn_bundle(SpriteSheetBundle {
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
        })
        .insert(Animation(idle_handle.clone()))
        .insert(AnimationState::default());
}
