use std::path::Path;

use crate::context::Context;
use crate::ext::PathExt;

pub fn link_entries_in_dir(ctx: &Context, base: &Path, path: &Path) {
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => {
            if ctx.cli.verbose {
                eprintln!("WARN: Unable to access {}", path.display());
            }
            return;
        }
    };

    for entry in entries {
        let entry_base = match entry {
            Ok(entry) => entry.path(),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to access entry in {}", path.display());
                }
                continue;
            }
        };

        let entry_target = match entry_base.replace_prefix(base, &ctx.target) {
            Ok(target) => target,
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Failed replacing prefix of {}", entry_base.display());
                }
                continue;
            }
        };

        link_file(ctx, &entry_base, &entry_target);
        link_dir(ctx, base, &entry_base, &entry_target);
    }
}

fn link_file(ctx: &Context, path: &Path, target: &Path) {
    if !path.is_file() {
        return;
    }

    if target.exists() {
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_file(path, target) {
            Ok(_) => (),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", path.display());
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", path.display(), target.display());
    }
}

fn link_dir(ctx: &Context, base: &Path, path: &Path, target: &Path) {
    if !path.is_dir() {
        return;
    }

    if target.exists() {
        link_entries_in_dir(ctx, base, path);
        return;
    }

    if !ctx.cli.no {
        match symlink::symlink_dir(path, target) {
            Ok(_) => (),
            Err(_) => {
                if ctx.cli.verbose {
                    eprintln!("WARN: Unable to create symlink for {}", path.display());
                }
                return;
            }
        }
    }

    if ctx.cli.verbose {
        println!("INFO: Link {} -> {}", path.display(), target.display());
    }
}
