use bevy::prelude::*;
use ordered_float::OrderedFloat;

use crate::structure::lib::structure::Structure;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildPlacementData {
    pub build_struct: Structure,
    pub build_ghost_id: Entity,
    pub build_struct_rotation: u16,
    pub click_delay: bool,
}
impl Default for BuildPlacementData {
    fn default() -> Self {
        Self {
            build_struct: Structure::default(),
            build_ghost_id: Entity::PLACEHOLDER,
            build_struct_rotation: 0,
            click_delay: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildSelectionData {
    pub selection_ui: Entity,
}
impl Default for BuildSelectionData {
    fn default() -> Self {
        Self {
            selection_ui: Entity::PLACEHOLDER,
        }
    }
}
#[derive(Debug, Clone, Component)]
pub struct SelectButtonData {
    pub structure: Structure,
}
