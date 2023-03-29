use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

pub static LOG_LEVEL: OnceCell<LogLv> = OnceCell::new();
pub static INST_COUNT: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
pub const LOG_ENABLE_INST: u64 = 3_5000_0000;

#[derive(PartialEq, Eq, PartialOrd)]
pub enum LogLv {
    NoLog,
    Diff,
    Info,
    Debug,
    Trace,
}

macro_rules! debugln {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Debug <= crate::log::LOG_LEVEL.get().unwrap() {
            std::println!($($rest)*);
        }
    }
}

macro_rules! debug {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Debug <= crate::log::LOG_LEVEL.get().unwrap() {
            std::print!($($rest)*);
        }
    }
}

macro_rules! infoln {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Info <= crate::log::LOG_LEVEL.get().unwrap() {
            std::println!($($rest)*);
        }
    }
}

#[allow(unused_macros)]
macro_rules! info {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Info <= crate::log::LOG_LEVEL.get().unwrap() {
            std::print!($($rest)*);
        }
    }
}

macro_rules! diffln {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Diff == crate::log::LOG_LEVEL.get().unwrap() &&
        *crate::log::INST_COUNT.lock().unwrap() > crate::log::LOG_ENABLE_INST {
            std::println!($($rest)*);
        }
    }
}

#[allow(unused_macros)]
macro_rules! diff {
    ($($rest:tt)*) => {
        if &crate::log::LogLv::Diff == crate::log::LOG_LEVEL.get().unwrap() &&
        crate::log::INST_COUNT.lock().unwrap() > crate::log::LOG_ENABLE_INST {
            std::print!($($rest)*);
        }
    }
}

#[allow(unused_imports)]
pub(crate) use {debug, debugln, diff, diffln, info, infoln};
