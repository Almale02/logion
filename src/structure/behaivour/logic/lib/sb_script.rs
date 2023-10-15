use bevy::{
    prelude::{Component},
    utils::HashMap,
};

use crate::{
    hashmap,
    lib::identifier::Identifier,
    resource::registry::sb_data_type_registry::SBDataTypeRegistry,
    structure::behaivour::{
        data::{
            data_types::{
                user_defined::SBUserDataType,
            },
        },
        logic::executing::lib::SBExecutableFun,
    },
};

use super::sb_exec_data::SBExecData;

#[derive(Clone, Debug, Component, Default)]
pub struct SBScript {
    pub group_id: Option<Identifier>,
    pub name: String,
    pub port_map: HashMap<u16, SBExecData>,
    pub exec_queue: Vec<SBExecData>,
}
impl SBScript {
    pub fn new_random_text(data_type_reg: &SBDataTypeRegistry) -> Self {
        let mut out = Self {
            group_id: None,
            name: "default".into(),
            port_map: HashMap::default(),
            exec_queue: Vec::default(),
        };

        let update_function: HashMap<String, SBExecutableFun> = hashmap! {

        "init".into() => SBExecutableFun::new(
            Identifier::new_function("game:hardware:text_display:new"),
            Some(hashmap!("self".into() => "change:self".into(),)),
            (&*data_type_reg, Identifier::new_data_type("game:hardware:text_display:new:in")).into(),
            Some("rand".into()),
        ),
        "rand".into() => SBExecutableFun::new(
            Identifier::new_function("game:u8:random"),
            Some(hashmap!("self".into() => "cast:self".into(),)),
            (&*data_type_reg, Identifier::new_data_type("game:u8:random:in")).into(),
            Some("cast".into())
        ),
        "cast".into() => SBExecutableFun::new(
            Identifier::new_function("game:u8:to_string"),
            Some(hashmap!("self".into() => "change:data".into(),)),
            (&*data_type_reg, Identifier::new_data_type("game:u8:to_string:in")).into(),
            Some("change".into())
        ),
        "change".into() => SBExecutableFun::new_consts(
            Identifier::new_function("game:hardware:text_display:change"),
            None,
            (&*data_type_reg, Identifier::new_data_type("game:hardware:text_display:change:in")).into(),
            None,
            None
        ),
        };
        let update_exec_data =
            SBExecData::new(update_function, SBUserDataType::default(), "init".into());

        out.port_map = hashmap!(
            69 => update_exec_data,
        );

        out
    }
    pub fn add_to_queue(&mut self, port: u16) {
        self.exec_queue
            .push(self.port_map.get(&port).unwrap().clone())
    }
}
impl PartialEq for SBScript {
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_id) = &self.group_id {
            if let Some(other_id) = &other.group_id {
                return self_id == other_id;
            }
        }
        false
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
*
*/
