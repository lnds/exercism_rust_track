#[macro_export]
macro_rules! hashmap {
    () => {
        {use std::collections::HashMap;
        HashMap::new() }
    };
    ( $($k:expr => $v:expr),* $(,)? )  => {
        {
            let mut temp_hash = HashMap::new();
            $(
                temp_hash.insert($k, $v);
            )*
            temp_hash
        }
    };
}
