use bevy::prelude::{ResMut, World};
use bevy::utils::HashMap;

use crate::{
    lib::identifier::Identifier,
    resource::registry::{
        sb_data_type_registry::SBDataTypeRegistry, sb_function_registry::SBFunctionRegistry,
    },
    structure::behaivour::{
        data::{
            data_types::{primitive::int::SBByte, user_defined::SBUserDataType},
            lib::SBDataType,
        },
        logic::lib::sb_function::SBFunction,
    },
};
use crate::{
    SBNumberGenericArithmetic, SBNumberGenericArithmeticReg, SBNumberGenericCasting,
    SBNumberGenericCastingReg,
};

pub struct SBTypeFunctionU8 {}
impl SBTypeFunctionU8 {
    pub fn register(
        mut function_registry: &mut ResMut<SBFunctionRegistry>,
        mut type_registry: &mut ResMut<SBDataTypeRegistry>,
    ) {
        SBNumberGenericArithmeticReg! {SBByte, "game:u8", function_registry, type_registry};
        SBNumberGenericCastingReg! {SBByte, "game:u8", function_registry, type_registry}
    }
}
SBNumberGenericArithmetic! {SBByte, u8, SBByte, "game:u8"}
SBNumberGenericCasting! {SBByte, "game:u8"}
