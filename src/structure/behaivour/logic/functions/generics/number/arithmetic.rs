



#[macro_export]
macro_rules! SBNumberGenericArithmetic {
    ($ty: ty, $raw_ty: ty, $ty_expr: expr, $type_id: expr) => {
        use crate::structure::behaivour::data::lib::IntoBoxSBDataType;
        use bevy::prelude::Entity;
        fn increment(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let value = crate::get_from_user_data!(input, "self", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(value + 1).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":increment:out")),
            )
        }
        fn decrement(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let value = crate::get_from_user_data!(input, "self", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(value - 1).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":decrement:out")),
            )
        }
        fn add(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let first = crate::get_from_user_data!(input, "self", $ty);
            let second = crate::get_from_user_data!(input, "other", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(first + second).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":decrement:out")),
            )
        }
        fn subtract(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let first = crate::get_from_user_data!(input, "self", $ty);
            let second = crate::get_from_user_data!(input, "other", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(first - second).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":decrement:out")),
            )
        }
        fn multiply(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let first = crate::get_from_user_data!(input, "self", $ty);
            let second = crate::get_from_user_data!(input, "other", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(first * second).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":decrement:out")),
            )
        }
        fn divide(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let first = crate::get_from_user_data!(input, "self", $ty);
            let second = crate::get_from_user_data!(input, "other", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(first / second).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":decrement:out")),
            )
        }

        fn zero(_world: &mut World, _input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), $ty_expr(0).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":zero:out")),
            )
        }
        fn random(_world: &mut World, _input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert(
                "self".to_owned(),
                $ty_expr(rand::random::<$raw_ty>()).boxed(),
            );

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":random:out")),
            )
        }
    };
}
