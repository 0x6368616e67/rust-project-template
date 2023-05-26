mod commands;
mod options;

use tracing::{info};
use tracing_subscriber::{self};
use clap::Parser;
use commands::Commands;
use options::Options;

const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--dirty", "--exclude", "*"],
            fallback = ""
        );

        if version.is_empty() {
            panic!("unable to query git revision");
        }
        version
    }
};
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION);

#[derive(Parser)]
#[clap(
    subcommand_required = true,
    arg_required_else_help = true,
    version = VERSION,
)]
struct CmdArgs {
    #[clap(flatten)]
    opts: Options,
    #[clap(subcommand)]
    commands: Commands,
}

fn init_log() {
    tracing_subscriber::fmt().event_format(
        tracing_subscriber::fmt::format()
        .with_file(true)
        .with_line_number(true)
    ).init();
}

fn main() -> Result<(), anyhow::Error>{
    init_log();
    
    let args = CmdArgs::parse();

    info!("execute with version {}", VERSION);


    let config_file = args.opts.config;
    match args.commands {
        Commands::Run(cmd) => cmd.run(&config_file),
    };

    Ok(())
}
