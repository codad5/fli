use crate::{fli::Fli, add, levenshtein_distance};

#[test]
pub fn test_add() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(5, 10), 15);
    assert_eq!(add(0, 0), 0);
}

#[test]
pub fn test_fli() {
    let mut fli = Fli::init("fli-test", "cook");
    fli.option("hello", "testing" , |app| {
        println!("Hello, World!");
        assert!(app.is_passed("hello".to_string()));
    });
    assert!(!fli.is_passed("test".to_owned()));
}


#[test]
pub fn test_get_callable_name_method(){
    let mut fli = Fli::init("fli-test", "cook");
    fli.option("-n --name", "testing", |_app| {});
    fli.option("-g --greet, <>","testing",  |_app| {});
    fli.option("-t --time, []", "testing", |_app| {});
    assert_eq!(fli.get_callable_name("-n".to_string()), "--name");
    assert_eq!(fli.get_callable_name("--greet".to_string()), "--greet");
    assert_eq!(fli.get_callable_name("time".to_string()), "--time");
    assert_eq!(fli.get_callable_name("g".to_string()), "--greet");
}



// test the levenshtein_distance function
#[test]
pub fn test_levenshtein_distance() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    assert_eq!(levenshtein_distance("hello", "world"), 4);
}

// test to make sure `Fli::init` is instantiating the struct correctly
#[test]
pub fn test_fli_init() {
    let fli = Fli::init("fli-test", "cook");
    assert_eq!(fli.get_app_name(), "fli-test");
}

// test if the `Fli::init_from_toml` is working correctly
#[test]
pub fn test_fli_init_from_toml() {
    let fli = Fli::init_from_toml();
    let toml_name = std::env::var("CARGO_PKG_NAME").unwrap();
    assert_eq!(fli.get_app_name(), toml_name);
}

