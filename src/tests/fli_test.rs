use crate::{fli::Fli, add};

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

