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
pub fn is_alpha(target_char: char) -> bool {
    target_char.is_ascii_alphabetic() || target_char == '_'
}

/// Checks whether the given char is ascii numeric.
/// else returns False.
/// # Arguments
/// * `target_char` - the character to check
pub fn is_numeric(target_char: char) -> bool {
    target_char.is_ascii_digit()
}

/// Checks whether given char is ascii numeric, or ascii alpabetic or _; return True.
/// else returns False.
/// # Arguments
/// * `target_char` - the character to check
pub fn is_alphanumeric(target_char: char) -> bool {
    is_alpha(target_char) || is_numeric(target_char)
}
