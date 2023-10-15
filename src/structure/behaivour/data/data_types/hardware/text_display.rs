use bevy::utils::HashMap;

use crate::{
    lib::identifier::Identifier,
    resource::registry::sb_data_type_registry::SBDataTypeInfo,
    structure::behaivour::data::lib::{IntoBoxSBDataType, SBDataType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBTextDisplay();
impl SBTextDisplay {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::DATA_TYPE, "game:hardware:text_display"),
            SBDataTypeInfo::new(None, Self().boxed()),
        );
    }
}
impl SBDataType for SBTextDisplay {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::DATA_TYPE, "game:hardware:text_display")
    }
}
