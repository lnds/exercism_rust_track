#[macro_export]
macro_rules! hashmap {
    ( $($k:expr => $v:expr),* $(,)? )  => {
        {   
            use std::collections::HashMap;
            let mut temp_hash = HashMap::new();
            $(
                temp_hash.insert($k, $v);
            )*
            temp_hash
        }
    };
}
