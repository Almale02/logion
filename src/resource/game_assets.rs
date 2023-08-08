use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct GameAssets {
    #[asset(path = "image", collection(mapped, typed))]
    image: HashMap<String, Handle<Image>>,
}
