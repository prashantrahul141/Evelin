#[macro_export]
macro_rules! die {
    ($($arg:tt)+) =>  {
        error!($($arg)+);
        panic!($($arg)+);
    }
}
