pub static LOG_LEVEL: LogLv = LogLv::NoLog;

#[derive(PartialEq, PartialOrd)]
pub enum LogLv {
    NoLog,
    Info,
    Debug,
    All
} 

macro_rules! debugln {
    ($($rest:tt)*) => {
        if crate::log::LogLv::Debug <= crate::log::LOG_LEVEL {
            std::println!($($rest)*);
        }
    }
}

macro_rules! debug {
    ($($rest:tt)*) => {
        if crate::log::LogLv::Debug <= crate::log::LOG_LEVEL {
            std::print!($($rest)*);
        }
    }
}

macro_rules! infoln {
    ($($rest:tt)*) => {
        if crate::log::LogLv::Info <= crate::log::LOG_LEVEL {
            std::println!($($rest)*);
        }
    }
}

#[allow(unused_macros)]
macro_rules! info {
    ($($rest:tt)*) => {
        if crate::log::LogLv::Info <= crate::log::LOG_LEVEL {
            std::print!($($rest)*);
        }
    }
}

#[allow(unused_imports)]
pub(crate) use {
    debug, 
    debugln,
    info,
    infoln,
};
