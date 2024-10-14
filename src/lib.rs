
#[cfg(not(doctest))]
pub mod fli;
pub mod macros;

pub use fli::Fli;
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

fn fli_default_callback(x: &Fli) {
    let command: Option<String> = x.get_arg_at(1);
    let command = match command {
        Some(c) => c,
        None => "".to_string(),
    };
    println!("Command not found: {}", command.bold().red());
    let err_msg_prefix = match command.len() {
        0 => "No",
        _ => "Invalid",
    };
    //  if command is not empty print similar command
    if command.len() <= 0 {
        x.print_help("No command provided");
        return;
    }
    println!("{0} Command {1} , use the '-h' or '--help' flag to see all command", err_msg_prefix, x.get_app_name().bold().red());
    x.print_most_similar_commands(command.as_str());
}
