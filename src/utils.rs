use std::path::Path;

use crate::context::Context;
use crate::ext::PathExt;

pub fn stow_entries_in_dir(ctx: &Context, base: &Path, path: &Path) {
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(err) => {
            if ctx.cli.verbose {
                eprintln!("WARN: Unable to access {}", path.display());
                if ctx.cli.debug {
                    eprintln!("{}", err);
                }
            }
            return;
        }
    };

    // TODO: Extract this part, so it can be reused in unstow function
    for entry in entries {
        let entry_path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to access entry in {}", path.display());
                    if ctx.cli.debug {
                        eprintln!("{}", err);
                    }
                }
                continue;
            }
        };

        let entry_target = match entry_path.replace_prefix(base, &ctx.target) {
            Ok(target) => target,
            Err(err) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Failed replacing prefix of {}", entry_path.display());
                    if ctx.cli.debug {
                        eprintln!("{}", err);
                    }
                }
                continue;
            }
        };

        stow_file(ctx, &entry_path, &entry_target);
        stow_dir(ctx, base, &entry_path, &entry_target);
    }
}

fn stow_file(ctx: &Context, path: &Path, target: &Path) {
    if !path.is_file() {
        return;
    }

    if target.is_file() {
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_file(path, target) {
            Ok(_) => (),
            Err(err) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", path.display());
                    if ctx.cli.debug {
                        eprintln!("{}", err);
                    }
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", path.display(), target.display());
    }
}

fn stow_dir(ctx: &Context, base: &Path, path: &Path, target: &Path) {
    if !path.is_dir() {
        return;
    }

    if target.is_dir() {
        stow_entries_in_dir(ctx, base, path);
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_dir(path, target) {
            Ok(_) => (),
            Err(err) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", path.display());
                    if ctx.cli.debug {
                        eprintln!("{}", err);
                    }
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", path.display(), target.display());
    }
}
