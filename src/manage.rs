use crate::{Config, Encryption};
use colored::*;
use std::env;
use structopt::StructOpt;
use std::path::PathBuf;

enum Mode {
    Create,
    Edit,
}

#[derive(Debug, StructOpt)]
pub struct Manage {
    /// Enforce editing of the configuration file. Scoob will error if the file does not exist
    #[structopt(short, long)]
    edit: bool,
    /// Enforce creation of the configuration file. Scoob will error if the file already exists
    #[structopt(short, long)]
    create: bool,
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

pub fn manage(cmd: &Manage) -> Result<(), &'static str> {
    if env::var("EDITOR").is_err()
        || env::var("EDITOR")
            .unwrap_or_else(|_| "".to_string())
            .is_empty()
    {
        return Err(
			"You must define your $EDITOR environment variable to edit a Scoob configuration file.",
		);
    }

    if cmd.create && cmd.edit {
        return Err("Both '--edit' and '--create' flags cannot be provided");
    }

    if cmd.create && Config::exists(&cmd.file) {
        return Err("The create flag was provided, but the secrets file already exists.");
    }

    if cmd.edit && !Config::exists(&cmd.file) {
        return Err("The edit flag was provided, but the secrets file does not exist.");
    }

    let mode: Mode = if cmd.create || !Config::exists(&cmd.file) {
        Mode::Create
    } else {
        Mode::Edit
    };

    let original_config = Config::get(&cmd.file);
    let encryption = Encryption {
        config: &original_config,
    };

    let temp_file_contents = match mode {
        Mode::Create => Config::default(),
        Mode::Edit => original_config.with_placeholders(),
    };

    let contents = edit::edit_with_builder(
        serde_yaml::to_string(&temp_file_contents).unwrap(),
        edit::Builder::new().suffix(".yml"),
    );

    let new_config: Config = serde_yaml::from_str(&contents.unwrap()).unwrap();

    let encrypted_config = encryption.encrypt_configuration(&new_config)?;

    std::fs::write(&cmd.file, serde_yaml::to_string(&encrypted_config).unwrap()).unwrap();

    println!("Wrote updated scoob configuration file at {:?}", cmd.file);

    Ok(())
}
