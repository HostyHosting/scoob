use crate::Modify;
use std::env;
use std::process::Command;

enum Mode {
	CREATE,
	EDIT,
}

pub fn modify(cmd: &Modify) -> Result<(), &'static str> {
	if cmd.create && cmd.edit {
		return Err("Both '--edit' and '--create' flags cannot be provided");
	}

	env::var("EDITOR").expect(
		"You must define your $EDITOR environment variable to modify a Scoob configuration file.",
	);

	let contents = edit::edit_with_builder("this is a test!", edit::Builder::new().suffix(".yml"));

	println!("contents: {:?}", contents);

	// if cmd.create {
	// 	std::fs::write(
	// 		&cmd.file,
	// 		serde_yaml::to_string(&Config::default_config()).expect("Failed to create default config"),
	// 	)
	// 	.unwrap();
	// }

	Ok(())
}
