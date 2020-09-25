use std::cmp::Ordering;

use crate::options::OrderBy;

#[inline]
pub fn compare<T: Ord>(t1: T, t2: T, order_by: OrderBy) -> Ordering {
    let (t1, t2) = match order_by {
        OrderBy::Asc => (t1, t2),
        OrderBy::Desc => (t2, t1),
    };

    t1.cmp(&t2)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_compare() {
        let a = 10;
        let b = 20;

        assert_eq!(compare(a, b, OrderBy::Asc), Ordering::Less);
    }
}
