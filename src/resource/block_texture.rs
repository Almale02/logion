use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource)]
pub struct BlockTexture {
    pub texture_map: HashMap<u8, String>,
}

impl Default for BlockTexture {
    fn default() -> Self {
        let mut map: HashMap<u8, String> = HashMap::default();

        map.insert(1, String::from("dirt/dirt.png"));
        map.insert(2, String::from("stone.png"));
        map.insert(3, String::from("dirt/grass_state/dirt_grass_top.png"));        

        BlockTexture { texture_map: map }

        // https://www.reddit.com/r/bevy/comments/y5fzkl/how_to_get_a_texture_from_a_in_memory_image_not_a/
    }   
}
