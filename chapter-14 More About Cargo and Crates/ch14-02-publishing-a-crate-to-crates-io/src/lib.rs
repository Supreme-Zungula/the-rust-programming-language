// making useful Documentation comments.
//-------------------------------------

/// Adds one to the number given.
///
/// ```
/// let arg  = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
}
