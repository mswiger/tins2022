use super::animation::{Animation, AnimationData, AnimationState};
use super::app::AppState;
use super::assets::GameAssets;
use super::map::{Map, TILE_HEIGHT, TILE_WIDTH};
use super::treasure::Treasure;
use benimator::Frame;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

const VEL_THRESHOLD: f32 = 0.001;

// How long to show message
const MSG_THRESHOLD: Duration = Duration::from_millis(200);

// Percentage of time that a message pops up when collecting treasure.
const MSG_FREQUENCY: u32 = 20;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Jumper {
    cooldown: bool,
    grounded: bool,
}

#[derive(Component)]
struct Message(Duration);

struct PlayerAnimations {
    idle: Handle<AnimationData>,
    walk: Handle<AnimationData>,
    jump: Handle<AnimationData>,
    swim: Handle<AnimationData>,
}

#[derive(PartialEq)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Component)]
struct PlayerDirection(Direction);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_player))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(player_movement)
                    .with_system(jump_reset)
                    .with_system(collect_treasure)
                    .with_system(despawn_messages),
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
            if animation.0 != animations.idle && jumper.grounded {
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
    } else if animation.0 == animations.walk && velocity.linvel.y < -VEL_THRESHOLD {
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

fn collect_treasure(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    treasure_query: Query<Entity, With<Treasure>>,
    message_query: Query<&Message>,
    mut collision_events: EventReader<CollisionEvent>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    let messages = vec![
        "CONSUME".to_string(),
        "MARRY\nAND\nREPRODUCE".to_string(),
        "CONFORM".to_string(),
        "NO\nINDEPENDENT\nTHOUGHT".to_string(),
        "DO\nNOT\nQUESTION\nAUTHORITY".to_string(),
        "WORK 8 HOURS\nSLEEP 8 HOURS\nPLAY 8 HOURS".to_string(),
        "HONOR\nAPATHY".to_string(),
    ];
    for event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            let player = player_query.single();
            let mut rng = rand::thread_rng();

            for treasure in treasure_query.iter() {
                if (h1 == &player && h2 == &treasure) || (h1 == &treasure && h2 == &player) {
                    commands.entity(treasure).despawn_recursive();

                    if message_query.iter().next().is_none() && rng.gen_range(0..100) < MSG_FREQUENCY {
                        let message = messages.choose(&mut rng).unwrap();
                        let mut node = commands.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            ..default()
                        });
                        node.add_children(|parent| {
                            parent
                                .spawn_bundle(
                                    // Create a TextBundle that has a Text with a single section.
                                    TextBundle::from_section(
                                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                                        message,
                                        TextStyle {
                                            font: game_assets.ui_font.clone(),
                                            font_size: 200.0,
                                            color: Color::BLACK,
                                        },
                                    ) // Set the alignment of the Text
                                    .with_text_alignment(TextAlignment::CENTER)
                                    .with_style(Style {
                                        align_self: AlignSelf::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    }),
                                )
                                .id()
                        });
                        node.insert(Message(time.time_since_startup()));
                        audio.play(game_assets.noise_sfx.clone());
                    } else {
                        audio.play(game_assets.coin_sfx.clone());
                    }
                }
            }
        }
    }
}

fn despawn_messages(
    mut commands: Commands,
    mut message_query: Query<(Entity, &mut Message)>,
    time: Res<Time>,
) {
    for (entity, message) in message_query.iter_mut() {
        if time.time_since_startup() - message.0 > MSG_THRESHOLD {
            commands.entity(entity).despawn_recursive();
        }
    }
}
