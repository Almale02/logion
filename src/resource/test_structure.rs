use bevy::prelude::Resource;

use crate::structure::lib::structure::Structure;

#[derive(Resource, Debug, Clone)]
pub struct TestStructure(pub Structure);

impl Default for TestStructure {
    fn default() -> Self {
        Self(Structure::default())
    }
}
