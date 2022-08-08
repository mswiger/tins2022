use super::map::Map;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 2,
        rows = 2,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/tiles.png")]
    pub tile_set_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 4,
        rows = 4,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/player.png")]
    pub player_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 2,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/piranha.png")]
    pub piranha_atlas: Handle<TextureAtlas>,

    #[asset(path = "textures/coin.png")]
    pub coin_image: Handle<Image>,

    #[asset(path = "maps/1.map")]
    pub map: Handle<Map>,

    #[asset(path = "music/Bonedust - When You Are Dead.mp3")]
    pub bgm: Handle<AudioSource>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub ui_font: Handle<Font>,

    #[asset(path = "sounds/noise.ogg")]
    pub noise_sfx: Handle<AudioSource>,

    #[asset(path = "sounds/coin.wav")]
    pub coin_sfx: Handle<AudioSource>,
}
