mod sts_core;

use anyhow::{Context, Result};
use std::{env, str};
use sts_core::{basic_edit, decrypt_save, encrypt_save};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let name: String;
    let source_name: &str = if args.len() < 2 {
        name = find_savefile()?;
        name.as_ref()
    } else {
        &args[1]
    };

    let target_name = if args.len() < 3 {
        source_name
    } else {
        &args[2]
    };

    println!("Parsing {} to {}", source_name, target_name);
    let mut game_map = decrypt_save(source_name)?;

    encrypt_save(target_name, basic_edit(&mut game_map))?;
    Ok(())
}

fn find_savefile() -> Result<String> {
    let file = glob::glob("**/*.autosave")?
        .next()
        .context("there is no matched file")??;
    let name = file.to_str().context("invalid filename")?;
    Ok(name.into())
}
