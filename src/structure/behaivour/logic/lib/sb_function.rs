use bevy::{prelude::*, utils::HashMap};

use crate::{
    lib::identifier::Identifier,
    resource::registry::{
        sb_data_type_registry::{SBDataTypeInfo, SBDataTypeRegistry},
        sb_function_registry::SBFunctionRegistry,
    },
    structure::behaivour::data::{data_types::user_defined::SBUserDataType, lib::SBDataType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SBFunction {
    pub function_id: Identifier,
    pub func: fn(&mut World, SBUserDataType, Entity) -> SBUserDataType,
}
impl Default for SBFunction {
    fn default() -> Self {
        Self {
            function_id: Identifier::default(),
            func: |_a, _b, _c| return SBUserDataType::default(),
        }
    }
}
impl SBFunction {
    pub fn register(
        function_registry: &mut ResMut<SBFunctionRegistry>,
        type_registry: &mut ResMut<SBDataTypeRegistry>,
        function: SBFunction,
        in_type_def: HashMap<String, Identifier>,
        out_type_def: HashMap<String, Identifier>,
        default_in: Box<dyn SBDataType>,
        default_out: Box<dyn SBDataType>,
    ) {
        let mut base = function.function_id.get_body().clone();

        base.push_str(":in");

        let input_type_id = Identifier::new(Identifier::DATA_TYPE, &base);

        base = function.function_id.get_body().clone();

        base.push_str(":out");

        let output_type_id = Identifier::new(Identifier::DATA_TYPE, &base);

        function_registry
            .0
            .insert(function.function_id.clone(), function.clone());
        type_registry.0.insert(
            input_type_id,
            SBDataTypeInfo::new(Some(in_type_def), default_in),
        );
        type_registry.0.insert(
            output_type_id,
            SBDataTypeInfo::new(Some(out_type_def), default_out),
        );
    }
    pub fn get_in_out_type(&self) -> (Identifier, Identifier) {
        let mut base = self.function_id.get_body().clone();

        base.push_str("in");

        let input_type_id = Identifier::new(Identifier::DATA_TYPE, &base);

        base = self.function_id.get_body().clone();

        base.push_str("out");

        let output_type_id = Identifier::new(Identifier::DATA_TYPE, &base);

        return (input_type_id, output_type_id);
    }
}
