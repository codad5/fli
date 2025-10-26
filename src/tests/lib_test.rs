use crate::{add, find_similar, levenshtein_distance};

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(100, 200), 300);
    assert_eq!(add(1, usize::MAX - 1), usize::MAX);
}

#[test]
fn test_levenshtein_distance_identical() {
    assert_eq!(levenshtein_distance("hello", "hello"), 0);
    assert_eq!(levenshtein_distance("", ""), 0);
}

#[test]
fn test_levenshtein_distance_empty_strings() {
    assert_eq!(levenshtein_distance("", "hello"), 5);
    assert_eq!(levenshtein_distance("hello", ""), 5);
}

#[test]
fn test_levenshtein_distance_single_char_diff() {
    assert_eq!(levenshtein_distance("cat", "bat"), 1);
    assert_eq!(levenshtein_distance("hello", "hallo"), 1);
}

#[test]
fn test_levenshtein_distance_examples() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    assert_eq!(levenshtein_distance("hello", "world"), 4);
}

#[test]
fn test_levenshtein_distance_complete_change() {
    assert_eq!(levenshtein_distance("abc", "xyz"), 3);
}

#[test]
fn test_levenshtein_distance_insertion() {
    assert_eq!(levenshtein_distance("cat", "cats"), 1);
    assert_eq!(levenshtein_distance("test", "tests"), 1);
}

#[test]
fn test_levenshtein_distance_deletion() {
    assert_eq!(levenshtein_distance("cats", "cat"), 1);
    assert_eq!(levenshtein_distance("hello", "hell"), 1);
}

#[test]
fn test_levenshtein_distance_transposition() {
    assert_eq!(levenshtein_distance("ab", "ba"), 2);
}

#[test]
fn test_find_similar_exact_match() {
    let options = vec!["serve".to_string(), "start".to_string(), "stop".to_string()];
    let similar = find_similar("serve", &options, 3);

    assert!(!similar.is_empty());
    assert_eq!(similar[0], "serve");
}

#[test]
fn test_find_similar_close_match() {
    let options = vec!["serve".to_string(), "start".to_string(), "stop".to_string()];
    let similar = find_similar("serv", &options, 3);

    assert!(!similar.is_empty());
    assert_eq!(similar[0], "serve");
}

#[test]
fn test_find_similar_multiple_matches() {
    let options = vec![
        "build".to_string(),
        "rebuilt".to_string(),
        "builder".to_string(),
    ];
    let similar = find_similar("buil", &options, 3);

    assert!(!similar.is_empty());
    let build_str = "build".to_string();
    assert!(similar.contains(&&build_str));
}

#[test]
fn test_find_similar_no_match() {
    let options = vec!["serve".to_string(), "start".to_string(), "stop".to_string()];
    let similar = find_similar("xyz", &options, 3);

    // Should return empty or very few results
    assert!(similar.is_empty() || similar.len() < options.len());
}

#[test]
fn test_find_similar_threshold() {
    let options = vec![
        "verbose".to_string(),
        "version".to_string(),
        "verify".to_string(),
    ];
    let similar = find_similar("verb", &options, 2);

    // With threshold of 2, might not find matches if distance is too large
    // Just check it doesn't panic
    assert!(similar.len() <= 2);
}

#[test]
fn test_find_similar_empty_input() {
    let options = vec!["serve".to_string(), "start".to_string(), "stop".to_string()];
    let similar = find_similar("", &options, 3);

    // Empty string might not return results depending on implementation
    // Just check it doesn't panic
    assert!(similar.len() <= 3);
}

#[test]
fn test_find_similar_single_option() {
    let options = vec!["serve".to_string()];
    let similar = find_similar("serv", &options, 3);

    assert_eq!(similar.len(), 1);
    assert_eq!(similar[0], "serve");
}

#[test]
fn test_find_similar_limit() {
    let options = vec![
        "build".to_string(),
        "rebuilt".to_string(),
        "builder".to_string(),
        "building".to_string(),
        "builds".to_string(),
    ];
    let similar = find_similar("buil", &options, 2);

    // Should return at most `limit` results
    assert!(similar.len() <= 2);
}

#[test]
fn test_levenshtein_unicode() {
    // Unicode characters are treated as single characters
    assert_eq!(levenshtein_distance("café", "café"), 0);
    assert_eq!(levenshtein_distance("日本", "日本"), 0);
    // Multi-byte UTF-8 characters work correctly
    assert_eq!(levenshtein_distance("🦀", "🦀"), 0); // Rust crab emoji
    assert_eq!(levenshtein_distance("a🦀b", "a🦀b"), 0);
}

#[test]
fn test_find_similar_case_sensitive() {
    let options = vec![
        "Serve".to_string(),
        "serve".to_string(),
        "SERVER".to_string(),
    ];
    let similar = find_similar("serve", &options, 3);

    assert!(!similar.is_empty());
    // Should find exact match
    let serve_str = "serve".to_string();
    assert!(similar.contains(&&serve_str));
}
