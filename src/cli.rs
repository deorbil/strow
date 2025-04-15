use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(action = ArgAction::Append)]
    pub package: Vec<PathBuf>,

    /// The directory that contains the package [default: current directory]
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// The directory where the package will be installed [default: parent of current directory]
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Simulate changes without making any modifications to filesystem
    #[arg(short, long, alias = "simulate")]
    pub no: bool,

    /// Print additional logging information
    #[arg(short, long)]
    pub verbose: bool,

    /// Print raw errors
    #[arg(long)]
    pub debug: bool,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
