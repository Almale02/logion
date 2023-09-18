use bevy::{prelude::Resource, utils::HashMap};

use crate::{
    lib::identifier::Identifier,
    structure::behaivour::data::{
        data_types::{primitive::int::*, primitive::string::SBString},
        lib::SBDataType,
    },
};

#[derive(Clone, Debug, Resource)]
pub struct SBDataTypeRegistry(pub HashMap<Identifier, SBDataTypeInfo>);

impl Default for SBDataTypeRegistry {
    fn default() -> Self {
        let mut out_map: HashMap<Identifier, SBDataTypeInfo> = HashMap::default();

        SBSignByte::register_type(&mut out_map);
        SBInt::register_type(&mut out_map);
        SBByte::register_type(&mut out_map);
        SBUint::register_type(&mut out_map);
        SBString::register_type(&mut out_map);

        SBDataTypeRegistry(out_map)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SBDataTypeInfo {
    pub field_map: Option<HashMap<String, Box<dyn SBDataType>>>,
    pub shorthand: String,
}
impl From<String> for SBDataTypeInfo {
    fn from(value: String) -> Self {
        Self {
            field_map: None,
            shorthand: value,
        }
    }
}
