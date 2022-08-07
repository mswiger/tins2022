mod animation;
mod app;
mod assets;
mod camera;
mod map;
mod player;

use animation::AnimationPlugin;
use app::AppState;
use assets::GameAssets;
use camera::CameraPlugin;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
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
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
