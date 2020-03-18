#[macro_export]
macro_rules! wrap {
    ($wrapper: tt, $($elem: expr),*) => {
        ($($wrapper::new($elem)),*)
    };

    ($wrapper: expr, $($elem: expr),*) => {
        ($($wrapper($elem)),*)
    };
}
