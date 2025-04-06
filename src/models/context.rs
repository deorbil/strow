use super::Cli;
use std::path::PathBuf;

pub struct LinkContext {
    pub cli: Cli,
    pub base_root_dir: PathBuf,
    pub target_root_dir: PathBuf,
}

impl LinkContext {
    pub fn new(cli: Cli, base_root_dir: PathBuf, target_root_dir: PathBuf) -> Self {
        Self {
            cli,
            base_root_dir,
            target_root_dir,
        }
    }
}
