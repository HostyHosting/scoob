use crate::encryption::Encryption;
use crate::{Config, Start, SubCommand};
use std::process::Command;

pub fn start(cmd: &Start) -> Result<(), &'static str> {
	let config = Config::get_config(&cmd.file);

	let mut sub_command = match &cmd.sub_command {
		SubCommand::Other(values) => values.iter(),
	};

	if sub_command.len() < 1 {
		// TODO: Error
		println!("Uh oh...")
	}

	let first_command = sub_command.next().unwrap();

	let mut command = Command::new(first_command);

	let encryption = Encryption { config };

	println!(
		"{:?}",
		encryption.encrypt(&"test".to_string(), &"test".to_string())
	);

	// for key in config.configuration.keys() {
	// 	command.env(
	// 		key,
	// 		config
	// 			.configuration
	// 			.get(key)
	// 			.expect("Unexpected missing configuration"),
	// 	);
	// }

	for arg in sub_command {
		command.arg(arg);
	}

	command.status().expect("Failed to start");

	Ok(())
}
