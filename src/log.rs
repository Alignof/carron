pub static LOG_LEVEL: LogLv = LogLv::Info;

#[derive(PartialEq, PartialOrd)]
pub enum LogLv {
    NoLog,
    Debug,
    Info,
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

macro_rules! info {
    ($($rest:tt)*) => {
        if crate::log::LogLv::Info <= crate::log::LOG_LEVEL {
            std::print!($($rest)*);
        }
    }
}

pub(crate) use {
    debug, 
    debugln,
    info,
    infoln,
};
