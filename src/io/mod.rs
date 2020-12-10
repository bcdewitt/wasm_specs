#[cfg_attr(target_arch = "wasm32", path = "log_wasm.rs")]
mod log;

pub use log::log;
