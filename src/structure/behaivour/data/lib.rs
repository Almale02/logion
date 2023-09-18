use std::collections::HashMap;

use dyn_clone::DynClone;
use mopa::Any;

use crate::lib::identifier::Identifier;

use super::data_types::user_defined::SBUserDataType;

pub trait SBDataType: Any + DynClone + Send + Sync {
    fn data_type_id(&self) -> Identifier;
}

dyn_clone::clone_trait_object!(SBDataType);
mopafy!(SBDataType);

impl std::fmt::Debug for dyn SBDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "temp")
    }
}
pub fn cast_sb_data_type<T: SBDataType>(data: T) -> Box<dyn SBDataType> {
    let b = Box::new(data);
    b
}
