pub mod fli;
pub use self::fli::*;

#[cfg(test)]
mod tests {
    // Import the test code from test.rs
    #[path = "test.rs"]
    mod test;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
