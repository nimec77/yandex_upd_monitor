#[cfg(feature = "logging")]
pub use log::{debug, error, info, trace, warn};

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "logging")]
pub fn init_logger() {
    env_logger::init();
    info!("Logging initialized");
}

#[cfg(not(feature = "logging"))]
pub fn init_logger() {
    // Do nothing when logging is disabled
}
