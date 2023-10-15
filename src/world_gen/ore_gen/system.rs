use ordered_float::OrderedFloat;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    lib::math::usvec2::USVec2, resource::level_data::LevelData,
    world_gen::ore_gen::lib::OrePatchData,
};
use bevy::prelude::*;

use super::lib::{OrePatchRoot, OrePatchSubpart};

pub fn gen_ore_patch_centers(mut level_data: ResMut<LevelData>) {
    let mut patch_positions: Vec<USVec2> = level_data.terrain_map.keys().cloned().collect();

    patch_positions.shuffle(&mut rand::thread_rng());
    let patch_positions = &mut patch_positions[0..200].to_vec();
    {
        //*patch_positions = vec![USVec2::new(6, level_data.change_y_smallest(9))];

        for val in patch_positions {
            let root_data = OrePatchRoot::new(5.);
            level_data.gen_grid[val.y][val.x].ore_patch = OrePatchData::Root(root_data.clone());

            // SECTION: Generate sub parts
            let width = rand::thread_rng().gen_range(4..=7);
            let height = rand::thread_rng().gen_range(4..=7);
            //let (width, height) = (5, 5);

            for y in val.y..val.y + width {
                for x in val.x..val.x + height {
                    if level_data.terrain_map.contains_key(&USVec2 { x, y }) {
                        let distance = OrePatchSubpart::calc_distance_from_center(USVec2::new(
                            ((width / 2) as i8 - (x - val.x) as i8).abs() as usize,
                            ((height / 2) as i8 - (y - val.y) as i8).abs() as usize,
                        )) as f32;
                        let multiplier = calc_multiplier(2, distance as usize);

                        level_data.gen_grid[y][x].ore_patch =
                            OrePatchData::SubPart(OrePatchSubpart {
                                patch_root: root_data.clone(),
                                multiplier,
                                distance_patch: distance as u8,
                            })
                    }
                }
            }
        }
    }
}
fn f(x: f32) -> f32 {
    let a: f32 = 6.;
    let b: f32 = 0.7;
    let c: f32 = 1.6;

    return f32::max(1.5, a * c.powf(b * -x));
}
fn calc_multiplier(max_distance: usize, distance: usize) -> OrderedFloat<f32> {
    // SECTION: smooth the ending
    if max_distance == distance && f(distance as f32) > 2.5 {
        return OrderedFloat(2.5);
    } else {
        return OrderedFloat(f(distance as f32));
    }
}
