#[cfg(test)]
mod tests;

pub mod process;
pub mod shuffle;
pub mod string;
/// Utilities to quickly check strings that represent urls
pub mod url;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
