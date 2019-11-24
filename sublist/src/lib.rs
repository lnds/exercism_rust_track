#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn comp_list<T: PartialEq>(short_list: &[T], long_list: &[T], result: Comparison) -> Comparison {
    if short_list.is_empty() || long_list.windows(short_list.len()).any(|l| l == short_list) {
        result
    } else {
        Comparison::Unequal
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() > second_list.len() {
        comp_list(second_list, first_list, Comparison::Superlist)
    } else {
        comp_list(first_list, second_list, Comparison::Sublist)
    }
}
