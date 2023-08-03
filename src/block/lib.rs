use bevy::prelude::*;

use crate::lib::Identifier::Identifier;
use std::collections::HashMap;

// SECTION: BLOCK_STATE
#[derive(Debug)]
pub struct BlockState {
    pub state_image: String,
}
// SECTION: BLOCK
pub trait Block {
    fn block_id(&self) -> Identifier;
    fn states(&self) -> HashMap<Identifier, BlockState>;
    fn state(&self) -> &Identifier;
    fn set_state(&mut self, value: Identifier);
}
// SECTOIN: GRASS_FACING
#[derive(Clone, Debug)]
pub enum GrassFacing {
    Top(String),
    TopLeft(String),
    TopRight(String),
    TopLeftRight(String),
}
// SECTION: BLOCK_RENDER_TYPE
pub enum BlockRenderType {
    BlockState(String),
    Generated(Handle<Image>)
}
// SECTON: HELPER_FN
pub fn add_block_state(
    map: &mut HashMap<Identifier, BlockState>,
    block_name: &str,
    state_name: &str,
    image: &str,
) {
    let id = format!(
        "{}",
        "blockstate:{".to_owned() + block_name + ":" + state_name + "}"
    );

    map.insert(
        Identifier { id },
        BlockState {
            state_image: image.to_string(),
        },
    );
}
pub fn add_grass_state(
    map: &mut HashMap<Identifier, BlockState>,
    block_name: &str,
    state_name: &str,
    image: &GrassFacing,
) {
    let img = match image {
        GrassFacing::Top(img)
        | GrassFacing::TopLeft(img)
        | GrassFacing::TopRight(img)
        | GrassFacing::TopLeftRight(img) => img,
    };
    let id = format!(
        "{}",
        "blockstate:{".to_owned() + block_name + ":" + state_name + "}"
    );

    map.insert(
        Identifier { id },
        BlockState {
            state_image: img.to_string(),
        },
    );
}
