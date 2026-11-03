/// Adds two to the given number.
///
/// # Example:
/// ```
/// let num = 3;
/// let result = add_two::add_two(num);
/// assert_eq!(5, result);
/// ```
pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
