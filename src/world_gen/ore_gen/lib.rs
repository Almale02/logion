
use ordered_float::OrderedFloat;

use crate::lib::math::usvec2::USVec2;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub enum OrePatchData {
    Root(OrePatchRoot),
    SubPart(OrePatchSubpart),
    #[default]
    None,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OrePatchSubpart {
    pub patch_root: OrePatchRoot,
    pub multiplier: OrderedFloat<f32>,
    pub distance_patch: u8,
}
impl OrePatchSubpart {
    pub fn calc_distance_from_center(pos: USVec2) -> u8 {
        let x = pos.x as u8;
        let y = pos.y as u8;
        return (u8::min(x, y) + u8::abs_diff(x, y)) as u8;
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OrePatchRoot {
    pub multiplier: OrderedFloat<f32>,
}
impl OrePatchRoot {
    pub fn new(multiplier: f32) -> Self {
        OrePatchRoot {
            multiplier: OrderedFloat(multiplier),
        }
    }
}
