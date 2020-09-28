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

const SIZES: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

pub fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    if bytes <= 1_f64 {
        return format!("{:.2} B", bytes);
    }
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), SIZES[i as usize])
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
