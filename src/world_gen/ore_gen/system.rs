use rand::seq::SliceRandom;

use crate::block::{block_type::BlockType, blocks::*, lib::*};
use crate::lib::USVec2::USVec2;
use crate::resource::level_data::LevelData;
use bevy::prelude::*;

fn gen_ore_patch_centers(mut level_data: ResMut<LevelData>) {
    for (y, row) in level_data.gen_grid.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {}
    }
}
