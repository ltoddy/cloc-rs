use std::cmp::Ordering;

pub fn compare<T: PartialOrd>(t1: T, t2: T) -> Ordering {
    if t1 < t2 {
        Ordering::Less
    } else if t1 > t2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
