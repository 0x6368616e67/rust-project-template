use clap::Parser;
use crate::run;


#[derive(Parser)]
pub enum Commands {
    /// Run exe
    Run(run::Cmd),
}