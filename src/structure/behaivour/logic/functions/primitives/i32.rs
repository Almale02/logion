use bevy::prelude::{ResMut, World};
use bevy::utils::HashMap;

use crate::structure::behaivour::data::data_types::primitive::int::SBInt;
use crate::{
    lib::identifier::Identifier,
    resource::registry::{
        sb_data_type_registry::SBDataTypeRegistry, sb_function_registry::SBFunctionRegistry,
    },
    structure::behaivour::{
        data::{data_types::user_defined::SBUserDataType, lib::SBDataType},
        logic::lib::sb_function::SBFunction,
    },
};
use crate::{
    SBNumberGenericArithmetic, SBNumberGenericArithmeticReg, SBNumberGenericCasting,
    SBNumberGenericCastingReg,
};

pub struct SBTypeFunctionI32 {}
impl SBTypeFunctionI32 {
    pub fn register(
        mut function_registry: &mut ResMut<SBFunctionRegistry>,
        mut type_registry: &mut ResMut<SBDataTypeRegistry>,
    ) {
        SBNumberGenericArithmeticReg! {SBInt, "game:i32", function_registry, type_registry};
        SBNumberGenericCastingReg! {SBInt, "game:i32", function_registry, type_registry};
    }
}
SBNumberGenericArithmetic! {SBInt, i32, SBInt, "game:i32"}
SBNumberGenericCasting! {SBInt, "game:i32"}
