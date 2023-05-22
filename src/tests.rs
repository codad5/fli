use super::*;

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(5, 10), 15);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_fli() {
    let mut fli = Fli::init("fli-test".to_owned());
    fli.option("hello".to_owned(), |app| {
        println!("Hello, World!");
        assert!(app.is_passed("hello".to_owned()));
    });
    fli.run();
    assert!(!fli.is_passed("test".to_owned()));
}
