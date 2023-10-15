#[macro_export]
macro_rules! get_from_user_data {
    ($data: expr, $key: expr, $ty: ty) => {
        $data
            .0
            .get($key)
            .unwrap()
            .downcast_ref::<$ty>()
            .unwrap()
            .0
            .clone()
    };
}
