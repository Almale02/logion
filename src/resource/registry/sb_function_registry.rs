use bevy::{
    prelude::{ResMut, Resource},
    utils::HashMap,
};

use crate::{
    lib::identifier::Identifier,
    structure::behaivour::logic::{
        functions::{
            hardwares::text_display::SBTypeFunctionTextDisplay,
            primitives::{
                i32::SBTypeFunctionI32, i8::SBTypeFunctionI8, u32::SBTypeFunctionU32,
                u8::SBTypeFunctionU8,
            },
        },
        lib::sb_function::SBFunction,
    },
};

use super::sb_data_type_registry::SBDataTypeRegistry;

#[derive(Clone, Debug, Resource)]
pub struct SBFunctionRegistry(pub HashMap<Identifier, SBFunction>);

impl Default for SBFunctionRegistry {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

pub fn function_registry(
    mut function_registry: ResMut<SBFunctionRegistry>,
    mut type_registry: ResMut<SBDataTypeRegistry>,
) {
    SBTypeFunctionU8::register(&mut function_registry, &mut type_registry);
    SBTypeFunctionU32::register(&mut function_registry, &mut type_registry);
    SBTypeFunctionI8::register(&mut function_registry, &mut type_registry);
    SBTypeFunctionI32::register(&mut function_registry, &mut type_registry);
    SBTypeFunctionTextDisplay::register(&mut function_registry, &mut type_registry)
}
