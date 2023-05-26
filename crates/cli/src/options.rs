use clap::Parser;
use std::path::PathBuf;
use std::ffi::OsStr;

#[derive(Parser)]
pub struct Options {
    /// Path for config file.
    #[clap(long, parse(from_os_str), default_value_os = OsStr::new("./config.toml"))]
    pub config: PathBuf,
}