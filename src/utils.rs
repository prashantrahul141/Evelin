/// Wrapper around error! and panic!, so that i dont have to call them individually.
#[macro_export]
macro_rules! die {
    ($($arg:tt)+) =>  {
        error!($($arg)+);
        panic!($($arg)+);
    }
}

/// Checks whether the given char is ascii alpabetic or _; return True.
/// else returns False.
/// # Arguments
/// * `target_char` - the character to check
///
/// # Examples
/// ```rust
/// use evelin::utils::*;
/// assert!(is_alpha('a'));
/// assert!(is_alpha('A'));
/// assert!(is_alpha('_'));
/// assert!(!is_alpha('1'));
/// ```
pub fn is_alpha(target_char: char) -> bool {
    target_char.is_ascii_alphabetic() || target_char == '_'
}

/// Checks whether the given char is ascii numeric.
/// else returns False.
/// # Arguments
/// * `target_char` - the character to check
///
/// # Examples
/// ```rust
/// use evelin::utils::*;
/// assert!(is_numeric('1'));
/// assert!(is_numeric('3'));
/// assert!(is_numeric('0'));
/// assert!(!is_numeric('A'));
/// ```
pub fn is_numeric(target_char: char) -> bool {
    target_char.is_ascii_digit()
}

/// Checks whether given char is ascii numeric, or ascii alpabetic or _; return True.
/// else returns False.
/// # Arguments
/// * `target_char` - the character to check
///
/// # Examples
/// ```rust
/// use evelin::utils::*;
/// assert!(is_alphanumeric('1'));
/// assert!(is_alphanumeric('a'));
/// assert!(is_alphanumeric('A'));
/// assert!(is_alphanumeric('_'));
/// assert!(!is_alphanumeric('@'));
/// ```
pub fn is_alphanumeric(target_char: char) -> bool {
    is_alpha(target_char) || is_numeric(target_char)
}
