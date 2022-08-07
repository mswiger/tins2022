use super::app::AppState;
use bevy::{prelude::*, reflect::TypeUuid};

#[derive(Debug, TypeUuid)]
#[uuid = "d48f497b-6f74-4369-9764-344fe2fae3f5"]
pub struct AnimationData(pub benimator::Animation);

#[derive(Default, Component, Deref)]
pub struct Animation(pub Handle<AnimationData>);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<AnimationData>()
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animate));
    }
}

fn animate(
    time: Res<Time>,
    animations: Res<Assets<AnimationData>>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut animation_state, mut texture, animation) in query.iter_mut() {
        let animation_data = animations.get(animation).unwrap();
        animation_state.update(&animation_data.0, time.delta());
        texture.index = animation_state.sprite_frame_index();
    }
}
