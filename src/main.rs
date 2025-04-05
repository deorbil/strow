mod ext;
mod models;

use ext::PathExt;
use models::{Cli, Context};
use std::io::Error;
use std::path::{Path, PathBuf};

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

    let ctx = Context::new(cli, package, target);
    if ctx.from == ctx.to {
        println!("ERROR: <TARGET> is the same as <DIR>!");
        return;
    }

    process(&ctx, &ctx.from);
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

fn process(ctx: &Context, base: &Path) {
    let entries = match base.read_dir() {
        Ok(entries) => entries,
        Err(_) => {
            if ctx.cli.verbose {
                eprintln!("WARN: Unable to access {}", base.display());
            }
            return;
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to access entry in {}", base.display());
                }
                continue;
            }
        };

        let target = match path.replace_prefix(&ctx.from, &ctx.to) {
            Ok(target) => target,
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Failed replacing prefix of {}", path.display());
                }
                continue;
            }
        };

        process_file(ctx, &path, &target);
        process_dir(ctx, &path, &target);
    }
}

fn process_file(ctx: &Context, base: &Path, target: &Path) {
    if !base.is_file() {
        return;
    }

    if target.exists() {
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_file(base, target) {
            Ok(_) => (),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", base.display());
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", base.display(), target.display());
    }
}

fn process_dir(ctx: &Context, base: &Path, target: &Path) {
    if !base.is_dir() {
        return;
    }

    if target.exists() {
        process(ctx, base);
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_dir(base, target) {
            Ok(_) => (),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", base.display());
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", base.display(), target.display());
    }
}
