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

    link_entries_in_dir(&ctx, &ctx.base_root_dir);
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

fn link_entries_in_dir(ctx: &LinkContext, base: &Path) {
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
        let entry_base = match entry {
            Ok(entry) => entry.path(),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to access entry in {}", base.display());
                }
                continue;
            }
        };

        let entry_target = match entry_base.replace_prefix(&ctx.base_root_dir, &ctx.target_root_dir)
        {
            Ok(target) => target,
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Failed replacing prefix of {}", entry_base.display());
                }
                continue;
            }
        };

        link_file(ctx, &entry_base, &entry_target);
        link_dir(ctx, &entry_base, &entry_target);
    }
}

fn link_file(ctx: &LinkContext, base: &Path, target: &Path) {
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

fn link_dir(ctx: &LinkContext, base: &Path, target: &Path) {
    if !base.is_dir() {
        return;
    }

    if target.exists() {
        link_entries_in_dir(ctx, base);
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
