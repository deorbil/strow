use std::path::PathBuf;

use crate::cli::Cli;

pub struct Context {
    pub cli: Cli,
    pub dir: PathBuf,
    pub target: PathBuf,
}

impl Context {
    pub fn new(cli: Cli, dir: PathBuf, target: PathBuf) -> Self {
        Self { cli, dir, target }
    }
}
