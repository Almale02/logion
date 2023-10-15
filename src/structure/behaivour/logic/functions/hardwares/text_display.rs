use crate::structure::behaivour::data::data_types::hardware::text_display::SBTextDisplay;
use crate::structure::behaivour::data::data_types::primitive::string::SBString;
use crate::structure::behaivour::data::lib::IntoBoxSBDataType;
use crate::structure::behaivour::hardware::hardwares::text_display::SBHTextDisplay;
use bevy::prelude::*;
use bevy::utils::HashMap;


use crate::{get_from_user_data, hashmap};
use crate::{
    lib::identifier::Identifier,
    resource::registry::{
        sb_data_type_registry::SBDataTypeRegistry, sb_function_registry::SBFunctionRegistry,
    },
    structure::behaivour::{
        data::{data_types::user_defined::SBUserDataType},
        logic::lib::sb_function::SBFunction,
    },
};

pub struct SBTypeFunctionTextDisplay {}
impl SBTypeFunctionTextDisplay {
    pub fn register(
        mut function_registry: &mut ResMut<SBFunctionRegistry>,
        mut type_registry: &mut ResMut<SBDataTypeRegistry>,
    ) {
        let type_id = "game:hardware:text_display";

        let mut in_type_self: HashMap<String, Identifier> = HashMap::default();
        let mut out_type_self: HashMap<String, Identifier> = HashMap::default();

        in_type_self.insert("self".to_owned(), Identifier::new_data_type(type_id));
        out_type_self.insert("self".to_owned(), Identifier::new_data_type(type_id));

        SBFunction::register(
            &mut function_registry,
            &mut type_registry,
            SBFunction {
                function_id: Identifier::new(
                    Identifier::FUNCTION,
                    format!("{type_id}:new").as_str(),
                ),
                func: new,
            },
            hashmap! {"self".to_owned() => Identifier::new_data_type(type_id),},
            hashmap! {"self".to_owned() => Identifier::new_data_type(type_id),},
            SBUserDataType::new(
                crate::hashmap!("self".to_string() => SBTextDisplay().boxed(),),
                Identifier::new_data_type(format!("{type_id}:new:in").as_str()),
            )
            .boxed(),
            SBUserDataType::new(
                crate::hashmap!("self".to_string() => SBTextDisplay().boxed(),),
                Identifier::new_data_type(format!("{type_id}:new:out").as_str()),
            )
            .boxed(),
        );
        SBFunction::register(
            &mut function_registry,
            &mut type_registry,
            SBFunction {
                function_id: Identifier::new(
                    Identifier::FUNCTION,
                    format!("{type_id}:change").as_str(),
                ),
                func: change,
            },
            hashmap! {
                "self".to_owned() => Identifier::new_data_type(type_id),
                "data".to_owned() => Identifier::new_data_type("game:string"),
            },
            hashmap! {"self".to_owned() => Identifier::new_data_type(type_id),},
            SBUserDataType::new(
                crate::hashmap!(
                    "self".to_string() => SBTextDisplay().boxed(),
                    "data".into() => SBString("default".into()).boxed(),
                ),
                Identifier::new_data_type(format!("{type_id}:new:in").as_str()),
            )
            .boxed(),
            SBUserDataType::new(
                crate::hashmap!("self".to_string() => SBTextDisplay().boxed(),),
                Identifier::new_data_type(format!("{type_id}:change:out").as_str()),
            )
            .boxed(),
        );
    }
}
fn new(_world: &mut World, _input: SBUserDataType, _structure: Entity) -> SBUserDataType {
    let mut out = SBUserDataType::default();
    out.1 = Identifier::new_data_type("game:hardware:text_display:new:out");
    out.0 = hashmap!("self".into() => SBTextDisplay().boxed(),);
    return out;
}
fn change(world: &mut World, input: SBUserDataType, structure: Entity) -> SBUserDataType {
    let text = get_from_user_data!(input, "data", SBString);
    world
        .get_entity_mut(structure)
        .unwrap()
        .get_mut::<SBHTextDisplay>()
        .unwrap()
        .text = text;

    // output self
    let mut out = SBUserDataType::default();
    out.1 = Identifier::new_data_type("game:hardware:text_display:change:out");
    out.0 = hashmap!("self".into() => SBTextDisplay().boxed(),);
    return out;
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
