use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug)]
struct BaseTester {
    path: PathBuf,
}

impl Default for BaseTester {
    fn default() -> Self {
        let mut dir = current_dir().unwrap();
        dir.push("tests/code");

        Self { path: dir }
    }
}

impl BaseTester {
    pub fn with_single_file(filename: &str) -> Self {
        let mut dir = current_dir().unwrap();
        dir.push(format!("tests/code/{}", filename));

        Self { path: dir }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use cloc::detail::Detail;
    use cloc::engine::Engine;

    #[test]
    pub fn test_rust_single_file() {
        let base = BaseTester::with_single_file("rust.rs");
        let engine = Engine::new(base.path);

        let actual = engine.calculate();

        let expected = Detail::new("Rust", 16, 7, 75);

        assert_eq!(expected, actual);
    }
}
