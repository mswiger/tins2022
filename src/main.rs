mod app;
mod assets;
mod map;
mod player;

use app::AppState;
use assets::GameAssets;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use map::MapPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Game)
                .with_collection::<GameAssets>(),
        )
        .add_state(AppState::Loading)
        .insert_resource(WindowDescriptor {
            title: "[GameDev] TINS 2022".to_string(),
            width: 1920.,
            height: 1080.,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: bevy::render::camera::OrthographicProjection {
            scale: 1./3.,
            ..default()
        },
        transform: Transform::from_xyz(280., 152., 999.),
        ..default()
    });
}
