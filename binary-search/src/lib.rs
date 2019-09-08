pub fn find(array: &[i32], key: i32) -> Option<usize> {
    match array.len() {
        0 => None,
        1 if key == array[0] => Some(0),
        1 => None,
        n => {
            let m = n / 2;
            if array[m] == key {
                Some(m)
            } else if array[m] > key {
                find(&array[..m], key)
            } else {
                find(&array[m..], key).map(|k| m + k)
            }
        }
    }
}
