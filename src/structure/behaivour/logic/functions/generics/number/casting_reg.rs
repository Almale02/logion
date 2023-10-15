#[macro_export]
macro_rules! SBNumberGenericCastingReg {
    ($ty_expr: expr, $type_id: expr, $function_registry: expr, $type_registry: expr) => {
        let mut in_type_self: HashMap<String, Identifier> = HashMap::default();
        let mut out_type_self: HashMap<String, Identifier> = HashMap::default();

        in_type_self.insert("self".to_owned(), Identifier::new_data_type($type_id));
        out_type_self.insert("self".to_owned(), Identifier::new_data_type($type_id));

        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":to_string")),
                func: to_string,
            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type("game:string"),},

            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":to_string:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => SBString("".into()).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":to_string:out"))).boxed(),
        );
    }
}
