use super::Cli;
use std::io::Error;
use std::path::PathBuf;

pub struct Context {
    pub cli: Cli,
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Context {
    pub fn new(cli: Cli) -> Result<Self, Error> {
        let from = resolve_package(&cli)?;
        let to = resolve_target(&cli)?;
        Ok(Self { cli, from, to })
    }
}

fn resolve_target(cli: &Cli) -> Result<PathBuf, Error> {
    let target = match &cli.target {
        Some(target) => target.clone(),
        None => {
            let dir = std::env::current_dir()?;
            match dir.parent() {
                Some(parent) => parent.to_path_buf(),
                None => dir,
            }
        }
    };
    dunce::canonicalize(target)
}

fn resolve_package(cli: &Cli) -> Result<PathBuf, Error> {
    let dir = match &cli.dir {
        Some(dir) => dir.clone(),
        None => std::env::current_dir()?,
    };
    dunce::canonicalize(dir.join(&cli.package))
}
