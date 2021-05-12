use crate::{Config, ConfigFile, Encryption, Modify};
use colored::*;
use std::env;

enum Mode {
	CREATE,
	EDIT,
}

pub fn modify(cmd: &Modify) -> Result<(), &'static str> {
	env::var("EDITOR").expect(
		"You must define your $EDITOR environment variable to modify a Scoob configuration file.",
	);

	if cmd.create && cmd.edit {
		return Err("Both '--edit' and '--create' flags cannot be provided");
	}

	if !cmd.create && !cmd.edit {
		println!("{}", "Neither create nor edit mode was provided. Scoob will attempt to automatically determine the correct mode.".yellow());
	}

	if cmd.create && Config::exists(&cmd.file) {
		return Err("The create flag was provided, but the secrets file already exists.");
	}

	if cmd.edit && !Config::exists(&cmd.file) {
		return Err("The edit flag was provided, but the secrets file does not exist.");
	}

	let mode: Mode = if cmd.create || !Config::exists(&cmd.file) {
		Mode::CREATE
	} else {
		Mode::EDIT
	};

	let original_config = Config::get_config(&cmd.file);
	let encryption = Encryption {
		config: original_config.clone(),
	};

	let temp_file_contents = match mode {
		Mode::CREATE => Config::default_config(),
		Mode::EDIT => original_config.with_placeholders(),
	};

	let contents = edit::edit_with_builder(
		serde_yaml::to_string(&temp_file_contents).unwrap(),
		edit::Builder::new().suffix(".yml"),
	);

	let new_config: ConfigFile = serde_yaml::from_str(&contents.unwrap()).unwrap();

	let encrypted_config = encryption.encrypt_configuration(new_config);

	std::fs::write(&cmd.file, serde_yaml::to_string(&encrypted_config).unwrap()).unwrap();

	println!("Wrote updated scoob configuration file at {:?}", cmd.file);

	Ok(())
}
