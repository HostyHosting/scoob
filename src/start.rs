use crate::{Config, Encryption, Start, SubCommand};
use std::process::Command;
use std::process::ExitStatus;

pub fn start(cmd: &Start) -> Result<ExitStatus, &'static str> {
	if !Config::exists(&cmd.file) {
		return Err("The provided configuration file does not exist");
	}

	let config = Config::get_config(&cmd.file);

	let mut sub_command = match &cmd.sub_command {
		SubCommand::Other(values) => values.iter(),
	};

	let first_command = sub_command.next().unwrap();

	let mut command = Command::new(first_command);

	let encryption = Encryption {
		config: config.clone(),
	};

	for (key, value) in config.configuration.iter() {
		command.env(key, encryption.decrypt(key, value));
	}

	for arg in sub_command {
		command.arg(arg);
	}

	let status = command.status().expect("Failed to start");

	Ok(status)
}
