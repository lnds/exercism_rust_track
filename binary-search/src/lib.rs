pub fn find<T, R>(array: R, key: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
    R: AsRef<[T]>,
{
    let ar = array.as_ref();
    match ar.len() {
        0 => None,
        1 if key == ar[0] => Some(0),
        1 => None,
        n => {
            let m = n / 2;
            if ar[m] == key {
                Some(m)
            } else if ar[m] > key {
                ar.get(..m).map(|l| find(l, key)).unwrap()
            } else {
                ar.get(m..).map(|r| find(r, key).map(|k| m + k)).unwrap()
            }
        }
    }
}
