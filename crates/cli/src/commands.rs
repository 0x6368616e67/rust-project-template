use clap::Parser;
use tracing::info;
use std::path::Path;

#[derive(Parser)]
pub enum Commands {
    /// Run exe
    Run(RunCmd),
}



#[derive(Parser)]
pub struct RunCmd {
    /// Customize RPC listening address
    #[clap(long)]
    rpc_addr: Option<String>,
}

impl RunCmd {
    pub fn run(self, config_file : &Path) {
        info!("run with config:{}", config_file.display());
    }
}