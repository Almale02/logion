use bevy::{prelude::Resource, utils::HashMap};

use crate::{
    hashmap,
    lib::{
        identifier::Identifier,
        nesting_tree::nesting_tree::{NTPath, NestingTree},
    },
    structure::{
        lib::structure_spawn_data::StructureSpawnData, predefined::general::PreStructGeneral,
    },
};

use super::sb_data_type_registry::SBDataTypeRegistry;

#[derive(Clone, Debug, Resource)]
pub struct StructureRegistry(pub HashMap<Identifier, StructureSpawnData>);
impl StructureRegistry {
    pub fn init(data_type_registry: &SBDataTypeRegistry) -> StructureRegistry {
        StructureRegistry(hashmap!(
            Identifier::new_structure("root:game:shape:ball") => PreStructGeneral::ball(data_type_registry),
            Identifier::new_structure("root:game:shape:square") => PreStructGeneral::square(data_type_registry),
            Identifier::new_structure("root:game:shape:triangle") => PreStructGeneral::triangle(data_type_registry),
        ))
    }
    pub fn get_nesting_tree(&mut self) -> NestingTree {
        let mut tree = NestingTree::default();

        for (key, val) in self.0.clone() {
            tree.add_node(NTPath::from_string(key.get_body()))
        }
        tree
    }
}
impl Default for StructureRegistry {
    fn default() -> Self {
        Self(HashMap::default())
    }
}
//PreStructGeneral::ball(),
//PreStructGeneral::triangle(),
//PreStructGeneral::square(),
