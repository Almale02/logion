use bevy::prelude::*;


use crate::structure::lib::{structure_spawn_data::StructureSpawnData};

#[derive(Clone, Debug, PartialEq)]
pub struct BuildPlacementData {
    pub struct_build_data: StructureSpawnData,
    pub build_ghost_id: Entity,
    pub build_struct_rotation: u16,
    pub click_delay: bool,
}
impl Default for BuildPlacementData {
    fn default() -> Self {
        Self {
            struct_build_data: StructureSpawnData::default(),
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
    pub spawn_data: StructureSpawnData,
}
