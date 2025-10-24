pub mod app;
pub mod command;
pub mod error;
pub mod macros;
pub mod option_parser;
pub use error::{FliError, Result};
pub use app::Fli;

use colored::Colorize;
#[cfg(test)]
pub mod tests;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[m][n]
}
