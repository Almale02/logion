use bevy::prelude::Resource;

use crate::structure::{lib::structure::Structure, predefined::general::PreStructGeneral};

#[derive(Clone, Debug, Resource)]
pub struct StructureRegistry(pub Vec<Structure>);
impl Default for StructureRegistry {
    fn default() -> Self {
        Self(vec![
            PreStructGeneral::ball(),
            PreStructGeneral::triangle(),
            PreStructGeneral::square(),
        ])
    }
}
