use std::time::SystemTime;

use chrono::{DateTime, Utc};

pub fn current_ts() -> String {
    let now: DateTime<Utc> = SystemTime::now().into();
    now.format("%F %T").to_string()
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("[{}] [INFO ] {}", $crate::logger::current_ts(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("[{}] [WARN ] {}", $crate::logger::current_ts(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("[{}] [ ERR ] {}", $crate::logger::current_ts(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("[{}] [DEBUG] {}", $crate::logger::current_ts(), format!($($arg)*))
    };
}
