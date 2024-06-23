#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (n, m) if n < m && is_sublist(first_list, second_list) => Comparison::Sublist,
        (n, m) if n > m && is_sublist(second_list, first_list) => Comparison::Superlist,
        _ if first_list == second_list => Comparison::Equal,
        _ => Comparison::Unequal,
    }
}

pub fn is_sublist<T: PartialEq>(candidate_sublist: &[T], candidate_superlist: &[T]) -> bool {
    if candidate_sublist.len() == 0 {
        // The empty list is a sublist of all lists.
        // Also, .windows(size) panics if given size=0, so this must be handled explicitly.
        return true;
    }
    candidate_superlist
        .windows(candidate_sublist.len())
        .any(|view| candidate_sublist == view)
}
