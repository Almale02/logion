use bevy::prelude::{ResMut, World};
use bevy::utils::HashMap;

use crate::structure::behaivour::data::data_types::primitive::int::{SBSignByte};
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

pub struct SBTypeFunctionI8 {}
impl SBTypeFunctionI8 {
    pub fn register(
        mut function_registry: &mut ResMut<SBFunctionRegistry>,
        mut type_registry: &mut ResMut<SBDataTypeRegistry>,
    ) {
        SBNumberGenericArithmeticReg! {SBSignByte, "game:i8", function_registry, type_registry};
        SBNumberGenericCastingReg! {SBSignByte, "game:i8", function_registry, type_registry};
    }
}

SBNumberGenericArithmetic! {SBSignByte, i8, SBSignByte, "game:i8"}
SBNumberGenericCasting! {SBSignByte, "game:i8"}
