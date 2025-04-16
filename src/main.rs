mod cli;
mod context;
mod ext;
mod utils;

use cli::Cli;
use context::Context;

use std::io::Error;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    let dir = match resolve_dir(&cli) {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("ERROR: Failed to resolve <DIR> directory");
            if cli.debug {
                eprintln!("{}", err);
            }
            return;
        }
    };

    let target = match resolve_target(&cli) {
        Ok(target) => target,
        Err(err) => {
            eprintln!("ERROR: Failed to resolve <TARGET> directory");
            if cli.debug {
                eprintln!("{}", err);
            }
            return;
        }
    };

    let ctx = Context::new(cli, dir, target);
    for package in &ctx.cli.package {
        // TODO: Extract this to a separate function
        if package.is_absolute() {
            eprintln!("ERROR: <PACKAGE> must be a relative path from <DIR>!");
            continue;
        }

        let path = ctx.dir.join(package);
        if path == ctx.target {
            eprintln!("ERROR: <TARGET> is the same as <PACKAGE>!");
            continue;
        }

        utils::stow_entries_in_dir(&ctx, &path, &path);
    }
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
