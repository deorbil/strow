mod ext;
mod models;

use ext::PathExt;
use models::{Cli, Context};
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    let ctx = match Context::new(cli) {
        Ok(ctx) => ctx,
        Err(_) => {
            eprintln!("ERROR: Unable to resolve package or target path");
            return;
        }
    };

    if ctx.from == ctx.to {
        println!("WARNING: Skipping target which is the same as current directory")
    }

    process(&ctx, &ctx.from);
}

fn process(ctx: &Context, base: &Path) {
    let entries = match base.read_dir() {
        Ok(entries) => entries,
        Err(_) => {
            if ctx.cli.verbose {
                eprintln!("ERROR: Unable to access {}", base.display());
            }
            return;
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("ERROR: Unable to access entry in {}", base.display());
                }
                continue;
            }
        };

        let target = match path.replace_prefix(&ctx.from, &ctx.to) {
            Ok(target) => target,
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("ERROR: Failed replacing prefix of {}", path.display());
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

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", base.display(), target.display());
    }

    if ctx.cli.no {
        return;
    }

    match symlink::symlink_file(base, target) {
        Ok(_) => (),
        Err(_) => {
            if ctx.cli.verbose {
                eprintln!("ERROR: Unable to create symlink for {}", base.display());
            }
        }
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

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", base.display(), target.display());
    }

    if ctx.cli.no {
        return;
    }

    match symlink::symlink_dir(base, target) {
        Ok(_) => (),
        Err(_) => {
            if ctx.cli.verbose {
                eprintln!("ERROR: Unable to create symlink for {}", base.display());
            }
        }
    }
}
