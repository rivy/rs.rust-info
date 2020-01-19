//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[derive(Debug, Clone, PartialEq, Copy)]
/// Rust channel type
pub enum RustChannel {
    /// Rust stable channel
    Stable,
    /// Rust beta channel
    Beta,
    /// Rust nightly channel
    Nightly,
}

#[derive(Debug, Clone, PartialEq)]
/// Holds the current rust installation and setup information
pub struct RustInfo {
    /// version
    pub version: Option<String>,
    /// channel
    pub channel: Option<RustChannel>,
    /// binary
    pub binary: Option<String>,
    /// commit-date
    pub commit_date: Option<String>,
    /// commit-hash
    pub commit_hash: Option<String>,
    /// host
    pub host: Option<String>,
    /// release
    pub release: Option<String>,
    /// LLVM version
    pub llvm_version: Option<String>,
    /// target arch cfg value
    pub target_arch: Option<String>,
    /// target endian cfg value
    pub target_endian: Option<String>,
    /// target env cfg value
    pub target_env: Option<String>,
    /// target family cfg value
    pub target_family: Option<String>,
    /// target features cfg value
    pub target_features: Option<Vec<String>>,
    /// target OS cfg value
    pub target_os: Option<String>,
    /// target pointer width cfg value
    pub target_pointer_width: Option<String>,
    /// target vendor cfg value
    pub target_vendor: Option<String>,
    /// target triple constructed from target arc, vendor, os and env
    pub target_triple: Option<String>,
}

impl RustInfo {
    /// Returns new instance
    pub fn new() -> RustInfo {
        RustInfo {
            version: None,
            channel: None,
            binary: None,
            commit_date: None,
            commit_hash: None,
            host: None,
            release: None,
            llvm_version: None,
            target_arch: None,
            target_endian: None,
            target_env: None,
            target_family: None,
            target_features: None,
            target_os: None,
            target_pointer_width: None,
            target_vendor: None,
            target_triple: None,
        }
    }
}
