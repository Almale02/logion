use crate::{lib::identifier::Identifier, structure::behaivour::data::lib::SBDataType};
use bevy::utils::HashMap;

use super::primitive::int::SBSignByte;

#[derive(Debug, Clone, Default)]
pub struct SBUserDataType(pub HashMap<String, Box<dyn SBDataType>>, pub Identifier);

impl SBUserDataType {
    pub fn new(data: HashMap<String, Box<dyn SBDataType>>, id: Identifier) -> Self {
        Self(data, id)
    }
}

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
