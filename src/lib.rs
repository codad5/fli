pub mod app;
pub mod command;
pub mod display;
pub mod error;
pub mod macros;
pub mod option_parser;
pub use app::Fli;
pub use error::{FliError, Result};

use colored::Colorize;
#[cfg(test)]
pub mod tests;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Calculate Levenshtein distance between two strings
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }
    
    matrix[len1][len2]
}



/// Find similar strings based on Levenshtein distance
pub fn find_similar<'a>(
    target: &str,
    candidates: &'a [String],
    max_distance: usize,
) -> Vec<&'a String> {
    candidates
        .iter()
        .filter(|candidate| levenshtein_distance(target, candidate) <= max_distance)
        .collect()
}
