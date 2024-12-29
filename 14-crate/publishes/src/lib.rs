//! # My Crate
//! 
//! `my_crate` is a collection of utilites to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Example
/// 
/// ```
/// let arg = 5;
/// let answer = publishes::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(input: i32) -> i32 {
    input + 1
}