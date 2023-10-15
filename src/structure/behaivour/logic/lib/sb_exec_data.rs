use bevy::utils::HashMap;

use crate::structure::behaivour::{
    data::data_types::user_defined::SBUserDataType, logic::executing::lib::SBExecutableFun,
};

#[derive(Debug, Clone, Default)]
pub struct SBExecData {
    pub function_pool: HashMap<String, SBExecutableFun>,
    pub init_data: SBUserDataType,
    pub first_function: String,
}
impl SBExecData {
    pub fn new(
        function_pool: HashMap<String, SBExecutableFun>,
        init_value: SBUserDataType,
        first_function: String,
    ) -> Self {
        Self {
            function_pool,
            init_data: init_value,
            first_function,
        }
    }
}
