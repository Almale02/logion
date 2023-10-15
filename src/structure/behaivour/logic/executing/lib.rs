use bevy::{
    prelude::*,
    utils::HashMap,
};

use crate::{
    lib::identifier::Identifier,
    resource::registry::{
        sb_function_registry::SBFunctionRegistry,
    },
    structure::{
        behaivour::{
            data::{
                data_types::{user_defined::SBUserDataType},
                lib::SBDataType,
            },
            logic::lib::{sb_exec_data::SBExecData},
        },
    },
};

#[derive(Debug, Clone)]
pub struct SBExecutionFrame {
    function_pool: HashMap<String, SBExecutableFun>,
    init_data: SBUserDataType,
    first_function: String,
    structure: Entity,
}
impl Default for SBExecutionFrame {
    fn default() -> Self {
        Self {
            function_pool: HashMap::default(),
            init_data: SBUserDataType::default(),
            first_function: String::default(),
            structure: Entity::PLACEHOLDER,
        }
    }
}
impl SBExecutionFrame {
    pub fn new(structure: Entity) -> Self {
        Self {
            structure,
            ..Default::default()
        }
    }
    fn execute_fn(&mut self, world: &mut World, exec_func: SBExecutableFun) -> SBUserDataType {
        let function_registry = world.get_resource::<SBFunctionRegistry>().unwrap().clone();

        let fun = function_registry
            .0
            .get(&exec_func.function_reg_data)
            .unwrap();

        let out = (fun.func)(world, exec_func.input, self.structure);

        // SECTION: output mapping to other functions
        if let Some(out_mapping) = exec_func.output_mappings {
            for (key, val) in out_mapping {
                let colon = val.find(":").unwrap();
                let func = &val[0..colon];
                let input = &val[1 + colon..];

                let target_func = self.function_pool.get_mut(func).unwrap();

                let target_input = target_func.input.0.get_mut(&input.to_string()).unwrap();
                dbg!(out.0.clone());
                *target_input = out.0.get(&key).unwrap().clone();
            }
        }
        out
    }
    pub fn execute_frame(&mut self, world: &mut World, exec_data: SBExecData) -> SBUserDataType {
        self.init_data = exec_data.init_data;
        self.first_function = exec_data.first_function;
        self.function_pool = exec_data.function_pool;

        let mut next_function = self.first_function.clone();
        for _i in 0..9999 {
            let exec_func = self.function_pool.get(&next_function).unwrap();
            let next_func = exec_func.next_function.clone();
            let out = self.execute_fn(world, exec_func.clone());

            match next_func {
                None => {
                    return out;
                }
                Some(x) => next_function = x.to_owned(),
            };
        }
        panic!("too long execution, probably a loop")
    }
}
#[derive(Debug, Clone, Default)]
pub struct SBExecutableFun {
    pub function_reg_data: Identifier,
    // INFO: the first shoud be the output name: self
    // the second should be the function pool id, and the input name: 123:self
    pub output_mappings: Option<HashMap<String, String>>,
    pub input: SBUserDataType,
    pub next_function: Option<String>,
    pub constants: Option<HashMap<String, Box<dyn SBDataType>>>,
}
impl SBExecutableFun {
    pub fn new(
        id: Identifier,
        output_mappings: Option<HashMap<String, String>>,
        input: Box<dyn SBDataType>,
        next: Option<String>,
    ) -> Self {
        let out = Self {
            function_reg_data: id,
            output_mappings,
            input: input.downcast_ref::<SBUserDataType>().unwrap().clone(),
            next_function: next,
            constants: None,
        };

        out
    }
    pub fn new_consts(
        id: Identifier,
        output_mappings: Option<HashMap<String, String>>,
        input: Box<dyn SBDataType>,
        next: Option<String>,
        constants: Option<HashMap<String, Box<dyn SBDataType>>>,
    ) -> Self {
        let mut out = Self {
            function_reg_data: id,
            output_mappings,
            input: input.downcast_ref::<SBUserDataType>().unwrap().clone(),
            next_function: next,
            constants,
        };

        if let Some(consts) = out.constants.clone() {
            for (key, val) in consts.iter() {
                out.input.0.insert(key.into(), val.clone());
            }
        }

        out
    }
}

/*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
* */
