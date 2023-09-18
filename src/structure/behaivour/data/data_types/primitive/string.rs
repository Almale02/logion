use bevy::utils::HashMap;

use crate::{
    lib::identifier::Identifier,
    resource::registry::sb_data_type_registry::SBDataTypeInfo,
    structure::behaivour::data::lib::{cast_sb_data_type, SBDataType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SBString(pub String);
impl SBString {
    pub fn register_type(registry_map: &mut HashMap<Identifier, SBDataTypeInfo>) {
        registry_map.insert(
            Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:string"),
            SBDataTypeInfo::from(String::from("string")),
        );
    }
}
impl SBDataType for SBString {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:string")
    }
    /*fn as_trait_object(&self) -> Box<dyn SBDataType> {
        Box::new(self.clone())
    }*/
}
fn a() {
    let mut data: Box<dyn SBDataType> = cast_sb_data_type(SBString("a".to_owned()));
    let str_data = data.downcast_ref::<SBString>().unwrap();

    let string = &str_data.0;

    data = cast_sb_data_type(SBString(string.repeat(5)))
}
