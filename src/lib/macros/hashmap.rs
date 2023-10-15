

#[macro_export]
macro_rules! hashmap {
    ($( $key:expr => $value:expr ),*,) => {{
        let mut map = bevy::utils::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}
