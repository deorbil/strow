use super::Cli;
use std::path::PathBuf;

pub struct Context {
    pub cli: Cli,
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Context {
    pub fn new(cli: Cli, from: PathBuf, to: PathBuf) -> Self {
        Self { cli, from, to }
    }
}
