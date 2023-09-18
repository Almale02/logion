use bevy::prelude::Component;

use crate::structure::behaivour::data::data_types::user_defined::SBUserDataType;

#[derive(Clone, Debug, Component, Default)]
pub struct StructureBehaviour {
    pub structure_data: SBUserDataType,
}
