#[macro_export]
macro_rules! SBNumberGenericArithmeticReg {
    ($ty_expr: expr, $type_id: expr, $function_registry: expr, $type_registry: expr) => {
        use crate::structure::behaivour::data::lib::IntoBoxSBDataType;
        let mut in_type_self: HashMap<String, Identifier> = HashMap::default();
        let mut out_type_self: HashMap<String, Identifier> = HashMap::default();

        in_type_self.insert("self".to_owned(), Identifier::new_data_type($type_id));
        out_type_self.insert("self".to_owned(), Identifier::new_data_type($type_id));

        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":increment")),
                func: increment,
            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":increment:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":increment:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":decrement")),
                func: decrement,
            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":decrement:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":decrement:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":zero")),
                func: zero,
            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":zero:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":zero:out"))).boxed(),
        );

        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":add")),
                func: add,
            },
            crate::hashmap! {
                "self".to_owned() => Identifier::new_data_type($type_id),
                "other".to_owned() => Identifier::new_data_type($type_id),

            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!(
                "self".to_string() => $ty_expr(0).boxed(),
                "other".to_string() => $ty_expr(0).boxed(),
            ),
                Identifier::new_data_type(concat!($type_id, ":add:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":add:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":subtract")),
                func: subtract,
            },
            crate::hashmap! {
                "self".to_owned() => Identifier::new_data_type($type_id),
                "other".to_owned() => Identifier::new_data_type($type_id),

            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!(
                "self".to_string() => $ty_expr(0).boxed(),
                "other".to_string() => $ty_expr(0).boxed(),
            ),
                Identifier::new_data_type(concat!($type_id, ":subtract:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":subtract:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":multiply")),
                func: multiply,
            },
            crate::hashmap! {
                "self".to_owned() => Identifier::new_data_type($type_id),
                "other".to_owned() => Identifier::new_data_type($type_id),

            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!(
                "self".to_string() => $ty_expr(0).boxed(),
                "other".to_string() => $ty_expr(0).boxed(),
            ),
                Identifier::new_data_type(concat!($type_id, ":multiply:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":multiply:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":divide")),
                func: divide,
            },
            crate::hashmap! {
                "self".to_owned() => Identifier::new_data_type($type_id),
                "other".to_owned() => Identifier::new_data_type($type_id),

            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!(
                "self".to_string() => $ty_expr(0).boxed(),
                "other".to_string() => $ty_expr(0).boxed(),
            ),
                Identifier::new_data_type(concat!($type_id, ":divide:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":divide:out"))).boxed(),
        );
        SBFunction::register(
            &mut $function_registry,
            &mut $type_registry,
            SBFunction {
                function_id: Identifier::new(Identifier::FUNCTION, concat!($type_id, ":random")),
                func: random,
            },
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},
            crate::hashmap! {"self".to_owned() => Identifier::new_data_type($type_id),},

            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":random:in"))).boxed(),
            SBUserDataType::new(crate::hashmap!("self".to_string() => $ty_expr(0).boxed(),),
                Identifier::new_data_type(concat!($type_id, ":random:out"))).boxed(),
        );
    };
}
