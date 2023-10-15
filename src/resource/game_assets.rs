

use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "image", collection(mapped, typed))]
    image: HashMap<String, Handle<Image>>,
}
impl Default for GameAssets {
    fn default() -> Self {
        GameAssets {
            image: HashMap::default(),
        }
    }
}
