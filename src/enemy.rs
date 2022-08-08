use super::animation::{Animation, AnimationData, AnimationState};
use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use super::player::Player;
use benimator::Frame;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

const ENEMY_VEL_MAGNITUDE: f32 = 30.;
const ENEMY_ATTACK_RADIUS: f32 = 80.;

#[derive(PartialEq)]
enum EnemyState {
    Roaming = 0,
    Attacking = 1,
}

#[derive(Component)]
struct Enemy {
    state: EnemyState,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_enemies))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(roaming_enemy_collision)
                    .with_system(update_enemies),
            );
    }
}

fn setup_enemies(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut animations: ResMut<Assets<AnimationData>>,
    maps: Res<Assets<Map>>,
) {
    let swim = AnimationData(benimator::Animation::from_frames(vec![
        Frame::new(0, Duration::from_millis(250)),
        Frame::new(1, Duration::from_millis(250)),
    ]));
    let swim_handle = animations.add(swim);

    let map = maps.get(&game_assets.map).unwrap();
    for enemy in map.enemies.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_assets.piranha_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(
                    TILE_WIDTH * enemy.0 as f32,
                    TILE_HEIGHT * enemy.1 as f32,
                    1.,
                )),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(Animation(swim_handle.clone()))
            .insert(AnimationState::default())
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(7., 7.))
            .insert(Velocity {
                linvel: Vec2::new(-ENEMY_VEL_MAGNITUDE, 0.),
                angvel: 0.,
            })
            .insert(Sensor)
            .insert(GravityScale(0.))
            .insert(Ccd::enabled())
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Enemy {
                state: EnemyState::Roaming,
            });
    }
}

fn roaming_enemy_collision(
    mut player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &mut Velocity, &mut TextureAtlasSprite)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        for (entity, enemy, mut velocity, mut sprite) in enemy_query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _flags) = event {
                if enemy.state == EnemyState::Roaming && (h1 == &entity || h2 == &entity) {
                    velocity.linvel.x = -velocity.linvel.x;
                    sprite.flip_x = !sprite.flip_x;
                }
            }
        }
    }
}

fn update_enemies(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &mut Enemy,
        &mut TextureAtlasSprite,
    )>,
) {
    let player_transform = player_query.single();
    for (enemy_transform, mut enemy_velocity, mut enemy, mut enemy_sprite) in enemy_query.iter_mut()
    {
        if enemy_transform
            .translation
            .distance(player_transform.translation)
            <= ENEMY_ATTACK_RADIUS
        {
            enemy.state = EnemyState::Attacking;
        }
        if enemy_transform
            .translation
            .distance(player_transform.translation)
            > ENEMY_ATTACK_RADIUS
        {
            enemy.state = EnemyState::Roaming;
            if enemy_velocity.linvel.x > 0. {
                enemy_velocity.linvel = Vec2::new(ENEMY_VEL_MAGNITUDE, 0.);
            } else {
                enemy_velocity.linvel = Vec2::new(-ENEMY_VEL_MAGNITUDE, 0.);
            }
        }

        if enemy.state == EnemyState::Attacking {
            let mut new_velocity = player_transform.translation - enemy_transform.translation;
            let magnitude = (new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y).sqrt();
            new_velocity.x = new_velocity.x * ENEMY_VEL_MAGNITUDE / magnitude;
            new_velocity.y = new_velocity.y * ENEMY_VEL_MAGNITUDE / magnitude;

            enemy_velocity.linvel = new_velocity.truncate();

            if new_velocity.x > 0. {
                enemy_sprite.flip_x = true;
            } else {
                enemy_sprite.flip_x = false;
            }
        }
    }
}
