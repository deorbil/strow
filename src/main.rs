mod cli;
mod context;
mod ext;
mod utils;

use cli::Cli;
use context::LinkContext;

use std::io::Error;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    if cli.package.is_absolute() {
        eprintln!("ERROR: <PACKAGE> must be a relative path from <DIR>!");
        return;
    }

    let target = match resolve_target(&cli) {
        Ok(target) => target,
        Err(_) => {
            eprintln!("ERROR: Failed to resolve <TARGET> directory");
            return;
        }
    };

    let package = match resolve_package(&cli) {
        Ok(package) => package,
        Err(_) => {
            eprintln!("ERROR: Failed to resolve <PACKAGE> directory");
            return;
        }
    };

    let ctx = LinkContext::new(cli, package, target);
    if ctx.base_root_dir == ctx.target_root_dir {
        println!("ERROR: <TARGET> is the same as <DIR>!");
        return;
    }

    utils::link_entries_in_dir(&ctx, &ctx.base_root_dir);
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
