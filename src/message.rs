#[derive(Debug)]
pub enum Message<T> {
    Content(T),
    End,
}
