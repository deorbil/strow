use std::path::Path;

use crate::context::LinkContext;
use crate::ext::PathExt;

pub fn link_entries_in_dir(ctx: &LinkContext, base: &Path) {
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
