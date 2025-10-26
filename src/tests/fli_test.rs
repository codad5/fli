use crate::{init_fli_from_toml, levenshtein_distance};

// test the levenshtein_distance function
#[test]
pub fn test_levenshtein_distance() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    assert_eq!(levenshtein_distance("hello", "world"), 4);
}

#[test]
pub fn test_can_create_fli_app() {
    let app = init_fli_from_toml!();
    assert_eq!(app.name, env!("CARGO_PKG_NAME"));
    assert_eq!(app.version, env!("CARGO_PKG_VERSION"));
}
