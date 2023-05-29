use std::ffi::OsStr;
use std::path::{
    PathBuf,
};

use clap::Parser;
use tracing::info;
use config::Config;


#[derive(Parser)]
pub struct Cmd {
    /// Path for config file.
    #[clap(long, parse(from_os_str), default_value_os = OsStr::new("./config.toml"))]
    pub config: PathBuf,
}

impl Cmd {
    pub fn run(self) {
        info!("run with config:{:?}", self.config);

        let cfg = Config::from_file(&self.config);
        info!("config is {:#?}", cfg);
    }
}