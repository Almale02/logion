use std::collections::HashMap;

use crate::{lib::identifier::Identifier, structure::behaivour::data::lib::SBDataType};

use super::primitive::int::SBSignByte;

#[derive(Debug, Clone, Default)]
pub struct SBUserDataType(HashMap<String, Box<dyn SBDataType>>, Identifier);

impl SBDataType for SBUserDataType {
    fn data_type_id(&self) -> Identifier {
        self.1.clone()
    }
}
fn a() {
    let mut data: Box<dyn SBDataType> = Box::new(SBUserDataType::default());
    data = Box::new(SBSignByte(1));
    data.data_type_id();
}
