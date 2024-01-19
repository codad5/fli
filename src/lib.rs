pub mod fli;
#[cfg(test)]
pub mod tests;
pub use self::fli::*;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
