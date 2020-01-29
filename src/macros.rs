#[macro_export]
macro_rules! unwrap_or_return {
    ($expr:expr, $default_val:expr) => {
        match $expr {
            Result::Ok(val) => val,
            Err(_) => {
                return $default_val;
            }
        }
    };
}
