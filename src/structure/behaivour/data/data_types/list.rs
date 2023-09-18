use crate::{lib::identifier::Identifier, structure::behaivour::data::lib::SBDataType};

use super::primitive::int::SBSignByte;

#[derive(Debug, Clone, Default)]
pub struct SBList(Vec<Box<dyn SBDataType>>);

impl SBDataType for SBList {
    fn data_type_id(&self) -> Identifier {
        Identifier::new(Identifier::BUILTIN_DATA_TYPE, "game:vec")
    }
}
impl PartialEq for SBList {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (i, val) in self.0.iter().enumerate() {
            if val.data_type_id() != other.0[i].data_type_id() {
                return false;
            }
        }
        true
    }
}
impl Eq for SBList {}
