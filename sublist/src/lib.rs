#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else {
        if first_list.len() > second_list.len() {
            let delta = first_list.len() - second_list.len();
            for d in 0..=delta {
                if &first_list[d..first_list.len() - delta + d] == second_list {
                    return Comparison::Superlist;
                }
            }
            Comparison::Unequal
        } else {
            let delta = second_list.len() - first_list.len();
            for d in 0..=delta {
                if &second_list[d..second_list.len() - delta + d] == first_list {
                    return Comparison::Sublist;
                }
            }
            Comparison::Unequal
        }
    }
}
