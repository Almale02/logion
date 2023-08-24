use crate::resource::level_data::LevelData;
use bevy::prelude::*;

fn gen_ore_patch_centers(level_data: ResMut<LevelData>) {
    for (_y, row) in level_data.gen_grid.iter().enumerate() {
        for (_x, _block) in row.iter().enumerate() {}
    }
}
