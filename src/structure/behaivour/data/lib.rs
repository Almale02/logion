

use bevy::prelude::World;
use dyn_clone::DynClone;
use mopa::Any;

use crate::{
    lib::identifier::Identifier, resource::registry::sb_data_type_registry::SBDataTypeRegistry,
};



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
impl From<(&World, Identifier)> for Box<dyn SBDataType> {
    fn from(value: (&World, Identifier)) -> Self {
        let registry = value.0.get_resource::<SBDataTypeRegistry>().unwrap();

        registry.0.get(&value.1).unwrap().default.clone()
    }
}
impl From<(&SBDataTypeRegistry, Identifier)> for Box<dyn SBDataType> {
    fn from(value: (&SBDataTypeRegistry, Identifier)) -> Self {
        value.0 .0.get(&value.1).unwrap().default.clone()
    }
}
pub trait IntoBoxSBDataType {
    fn boxed(self) -> Box<dyn SBDataType>;
}

impl<T: SBDataType> IntoBoxSBDataType for T {
    fn boxed(self) -> Box<dyn SBDataType> {
        Box::new(self) as Box<dyn SBDataType>
    }
}
