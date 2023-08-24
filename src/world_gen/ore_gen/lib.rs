use ordered_float::OrderedFloat;

use crate::lib::USVec2::USVec2;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum OrePatchData {
    Root { multiplier: OrderedFloat<f32> },
    SubPart(OrePatchSubpart),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OrePatchSubpart {
    patch_ref: Box<OrePatchData>,
    multiplier: OrderedFloat<f32>,
    distance_patch: u8,
}
impl OrePatchSubpart {
    fn calc_distance_from_center(pos: USVec2) -> u8 {
        let x = pos.x as u8;
        let y = pos.y as u8;
        return (u8::min(x, y) + u8::abs_diff(x, y)) as u8;
    }
}
