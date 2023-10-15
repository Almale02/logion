use bevy::prelude::*;

use super::build_action::lib::{BuildPlacementData, BuildSelectionData};

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct ActionStateData {
    pub build_placement_data: BuildPlacementData,
    pub build_selection_data: BuildSelectionData,
}
impl Default for ActionStateData {
    fn default() -> Self {
        ActionStateData {
            build_placement_data: BuildPlacementData::default(),
            build_selection_data: BuildSelectionData::default(),
        }
    }
}
