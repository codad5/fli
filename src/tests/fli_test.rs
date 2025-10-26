use crate::{levenshtein_distance};


// test the levenshtein_distance function
#[test]
pub fn test_levenshtein_distance() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    assert_eq!(levenshtein_distance("hello", "world"), 4);
}
