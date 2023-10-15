



#[macro_export]
macro_rules! SBNumberGenericCasting {
    ($ty: ty, $type_id: expr) => {
        use crate::structure::behaivour::data::data_types::primitive::string::SBString;
        fn to_string(_world: &mut World, input: SBUserDataType, _entity: Entity) -> SBUserDataType {
            let value = crate::get_from_user_data!(input, "self", $ty);

            let mut out_map: HashMap<String, Box<dyn SBDataType>> = HashMap::default();
            out_map.insert("self".to_owned(), SBString(value.to_string()).boxed());

            SBUserDataType(
                out_map,
                Identifier::new(Identifier::DATA_TYPE, concat!($type_id, ":to_string:out")),
            )
        }
    };
}
fn a() {}
