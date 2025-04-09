#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if is_contain(a, b) {
        Comparison::Superlist
    } else if is_contain(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn is_contain<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    }
    if a.starts_with(b) {
        return true;
    }
    is_contain(&a[1..], b)
}
