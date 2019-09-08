
pub fn find<T>(array: &[T], key: T) -> Option<usize>
where 
    T:  PartialEq+PartialOrd+Clone,
{
    match array.len() {
        0 => None,
        1 if key == array[0] => Some(0),
        1 => None,
        n => {
            let m = n / 2;
            if array[m] == key {
                Some(m)
            } else if array[m] > key {
                let (l, _) = array.split_at(m);
                find(l, key)
            } else {
                let (_, r) = array.split_at(m);
                find(r, key).map(|k| m + k)
            }
        }
    }
}
