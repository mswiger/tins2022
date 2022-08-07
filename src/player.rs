use super::animation::{Animation, AnimationData, AnimationState};
use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use benimator::Frame;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

const VEL_THRESHOLD: f32 = 0.001;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Jumper {
    cooldown: bool,
    grounded: bool,
}

struct PlayerAnimations {
    idle: Handle<AnimationData>,
    walk: Handle<AnimationData>,
    jump: Handle<AnimationData>,
    swim: Handle<AnimationData>,
}

pub struct PlayerPlugin;

#[derive(PartialEq)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Component)]
struct PlayerDirection(Direction);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_player))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(player_movement)
                    .with_system(jump_reset),
            );
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
        .insert(Player)
        .insert(PlayerDirection(Direction::Left))
        .insert(Jumper {
            cooldown: true,
            grounded: true,
        })
        .insert(Animation(idle_handle.clone()))
        .insert(AnimationState::default())
        .insert(Collider::cuboid(7.0, 8.0))
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut players: Query<
        (
            &mut Transform,
            &mut Animation,
            &mut AnimationState,
            &mut TextureAtlasSprite,
            &mut Velocity,
            &mut PlayerDirection,
            &mut Jumper,
        ),
        (With<RigidBody>, With<Player>),
    >,
) {
    for (
        mut transform,
        mut animation,
        mut animation_state,
        mut sprite,
        mut velocity,
        mut direction,
        mut jumper,
    ) in players.iter_mut()
    {
        if keyboard_input.pressed(KeyCode::Space) {
            if !jumper.cooldown {
                velocity.linvel = Vec2::new(0., 50.);
                jumper.cooldown = true;
                jumper.grounded = false;
                animation.0 = animations.jump.clone();
                animation_state.reset();
            }
        } else {
            jumper.cooldown = false;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 0.2;
            if direction.0 != Direction::Left {
                direction.0 = Direction::Left;
                sprite.flip_x = false;
            }
            update_move_anim(
                &mut animation,
                &mut animation_state,
                &velocity,
                &jumper,
                &animations,
            );
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 0.2;
            if direction.0 != Direction::Right {
                direction.0 = Direction::Right;
                sprite.flip_x = true;
            }
            update_move_anim(
                &mut animation,
                &mut animation_state,
                &velocity,
                &jumper,
                &animations,
            );
        } else {
            if animation.0 == animations.idle && jumper.grounded {
                animation.0 = animations.idle.clone();
                animation_state.reset();
            } else if animation.0 == animations.jump && velocity.linvel.y < VEL_THRESHOLD {
                animation.0 = animations.swim.clone();
                animation_state.reset();
            }
        }
    }
}

fn update_move_anim(
    animation: &mut Animation,
    animation_state: &mut AnimationState,
    velocity: &Velocity,
    jumper: &Jumper,
    animations: &PlayerAnimations,
) {
    if animation.0 == animations.idle {
        animation.0 = animations.walk.clone();
        animation_state.reset();
    } else if !jumper.grounded
        && animation.0 == animations.jump
        && velocity.linvel.y < VEL_THRESHOLD
    {
        animation.0 = animations.swim.clone();
        animation_state.reset();
    }
}

fn jump_reset(
    mut query: Query<(
        Entity,
        &mut Jumper,
        &Velocity,
        &mut Animation,
        &mut AnimationState,
    )>,
    mut collision_events: EventReader<CollisionEvent>,
    animations: Res<PlayerAnimations>,
) {
    for event in collision_events.iter() {
        for (entity, mut jumper, velocity, mut animation, mut animation_state) in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _flags) = event {
                if h1 == &entity || h2 == &entity {
                    if velocity.linvel.y < VEL_THRESHOLD && !jumper.grounded {
                        jumper.cooldown = false;
                        jumper.grounded = true;
                        animation.0 = animations.idle.clone();
                        animation_state.reset();
                    }
                }
            }
        }
    }
}
