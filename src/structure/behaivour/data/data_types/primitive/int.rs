use bevy::utils::HashMap;

use crate::{
    lib::identifier::Identifier, resource::registry::sb_data_type_registry::SBDataTypeInfo,
    structure::behaivour::data::lib::SBDataType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBSignByte(pub i8);
impl SBSignByte {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:i8"),
            SBDataTypeInfo::from(String::from("i8")),
        );
    }
}
impl SBDataType for SBSignByte {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:i8")
    }
}
// SECTION:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBInt(pub i32);
impl SBInt {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:i32"),
            SBDataTypeInfo::from(String::from("i32")),
        );
    }
}
impl SBDataType for SBInt {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:i32")
    }
}
// SECTION:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBByte(pub u8);
impl SBByte {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:u8"),
            SBDataTypeInfo::from(String::from("u8")),
        );
    }
}
// SECTION:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBUint(pub u32);
impl SBUint {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:u32"),
            SBDataTypeInfo::from(String::from("u32")),
        );
    }
}
impl SBDataType for SBUint {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:u32")
    }
}
