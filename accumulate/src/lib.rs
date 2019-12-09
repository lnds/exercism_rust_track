pub fn map<F, T, R>(input: Vec<T>, mut function: F) -> Vec<R>
where
    F: FnMut(T) -> R,
{
        let mut result = vec![];
    for i in input {
        result.push(function(i))
    }
    result
}
