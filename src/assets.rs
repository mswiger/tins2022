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
        columns = 1,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/player.png")]
    pub player_atlas: Handle<TextureAtlas>,

    #[asset(path = "maps/1.map")]
    pub map: Handle<Map>,
}
