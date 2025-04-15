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

    let dir = match resolve_dir(&cli) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("ERROR: Failed to resolve <DIR> directory");
            return;
        }
    };

    let target = match resolve_target(&cli) {
        Ok(target) => target,
        Err(_) => {
            eprintln!("ERROR: Failed to resolve <TARGET> directory");
            return;
        }
    };

    // TODO: Extract this to a separate function
    if cli.package.is_absolute() {
        eprintln!("ERROR: <PACKAGE> must be a relative path from <DIR>!");
        return;
    }

    let package = dir.join(&cli.package);

    let ctx = LinkContext::new(cli, package, target);
    if ctx.base_root_dir == ctx.target_root_dir {
        println!("ERROR: <TARGET> is the same as <DIR>!");
        return;
    }

    utils::link_entries_in_dir(&ctx, &ctx.base_root_dir);
}

fn resolve_dir(cli: &Cli) -> Result<PathBuf, Error> {
    let dir = match &cli.dir {
        Some(dir) => dir.clone(),
        None => std::env::current_dir()?,
    };
    dunce::canonicalize(dir)
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
