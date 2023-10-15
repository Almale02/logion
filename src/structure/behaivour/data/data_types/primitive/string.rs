use bevy::utils::HashMap;

use crate::{
    lib::identifier::Identifier,
    resource::registry::sb_data_type_registry::SBDataTypeInfo,
    structure::behaivour::data::lib::{IntoBoxSBDataType, SBDataType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBString(pub String);
impl SBString {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::DATA_TYPE, "game:string"),
            SBDataTypeInfo::new(None, Self("default".to_owned()).boxed()),
        );
    }
}
impl SBDataType for SBString {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::DATA_TYPE, "game:string")
    }
}
fn a() {
    let mut data: Box<dyn SBDataType> = SBString("a".to_owned()).boxed();
    let str_data = data.downcast_ref::<SBString>().unwrap();

    let string = &str_data.0;

    data = SBString(string.repeat(5)).boxed()
}
