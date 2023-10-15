use bevy::{
    prelude::{ResMut, Resource},
    utils::HashMap,
};

use crate::{
    lib::identifier::Identifier,
    structure::behaivour::data::{
        data_types::{
            hardware::text_display::SBTextDisplay, primitive::int::*, primitive::string::SBString,
        },
        lib::{IntoBoxSBDataType, SBDataType},
    },
};

#[derive(Clone, Debug, Resource)]
pub struct SBDataTypeRegistry(pub HashMap<Identifier, SBDataTypeInfo>);

impl Default for SBDataTypeRegistry {
    fn default() -> Self {
        let out_map: HashMap<Identifier, SBDataTypeInfo> = HashMap::default();

        SBDataTypeRegistry(out_map)
    }
}

#[derive(Debug, Clone)]
pub struct SBDataTypeInfo {
    pub type_map: Option<HashMap<String, Identifier>>,
    pub shorthand: String,
    pub default: Box<dyn SBDataType>,
}
impl SBDataTypeInfo {
    pub fn new(
        _type_map: Option<HashMap<String, Identifier>>,
        default: Box<dyn SBDataType>,
    ) -> Self {
        Self {
            type_map: None,
            shorthand: String::from("temp"),
            default,
        }
    }
}
impl Default for SBDataTypeInfo {
    fn default() -> Self {
        Self {
            type_map: None,
            shorthand: String::default(),
            default: SBByte(0).boxed(),
        }
    }
}
pub fn data_type_register(mut registry: ResMut<SBDataTypeRegistry>) {
    let mut out_map = &mut registry.0;

    SBSignByte::register_type(&mut out_map);
    SBInt::register_type(&mut out_map);
    SBByte::register_type(&mut out_map);
    SBUint::register_type(&mut out_map);
    SBString::register_type(&mut out_map);
    SBTextDisplay::register_type(&mut out_map);
}
