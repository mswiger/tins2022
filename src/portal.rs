use super::animation::{Animation, AnimationData, AnimationState};
use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use super::treasure::Treasure;
use benimator::Frame;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Portal {
    pub opened: bool
}

pub struct PortalAnimations {
    opened: Handle<AnimationData>,
    closed: Handle<AnimationData>,
}

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_portal))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_portal));
    }
}

fn setup_portal(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    maps: Res<Assets<Map>>,
    mut animations: ResMut<Assets<AnimationData>>,
) {
    let map = maps.get(&game_assets.map).unwrap();

    let closed = AnimationData(benimator::Animation::from_frames(vec![Frame::new(
        0,
        Duration::from_millis(250),
    )]));
    let closed_handle = animations.add(closed);

    let opened = AnimationData(benimator::Animation::from_frames(vec![
        Frame::new(2, Duration::from_millis(250)),
        Frame::new(1, Duration::from_millis(250)),
    ]));
    let opened_handle = animations.add(opened);

    commands.insert_resource(PortalAnimations {
        opened: opened_handle.clone(),
        closed: closed_handle.clone(),
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.portal_atlas.clone(),
            transform: Transform::from_translation(Vec3::new(
                TILE_WIDTH * map.portal_spawn.0 as f32,
                TILE_HEIGHT * map.portal_spawn.1 as f32,
                1.,
            )),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(Animation(closed_handle.clone()))
        .insert(AnimationState::default())
        .insert(Collider::cuboid(7., 7.))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Portal {
            opened: false
        });
}

fn update_portal(
    treasure_query: Query<Entity, With<Treasure>>,
    mut portal_query: Query<(&mut Portal, &mut Animation, &mut AnimationState)>,
    portal_animations: Res<PortalAnimations>,
    game_assets: Res<GameAssets>,
    audio: Res<Audio>,
) {
    let (mut portal, mut animation, mut animation_state) = portal_query.single_mut();
    if treasure_query.iter().next().is_none() && !portal.opened {
        animation.0 = portal_animations.opened.clone();
        animation_state.reset();
        portal.opened = true;
        audio.play(game_assets.portal_sfx.clone());
    }
}
