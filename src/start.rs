use crate::{Config, Encryption, Start, SubCommand};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn start(cmd: &Start) -> Result<i32, &'static str> {
    if !Config::exists(&cmd.file) {
        return Err("The provided configuration file does not exist");
    }

    let config = Config::get(&cmd.file);

    let mut sub_command = match &cmd.sub_command {
        SubCommand::Other(values) => values.iter(),
    };

    let first_command = sub_command.next();

    if first_command.is_none() {
        return Err("No command was provided.");
    }

    let mut command = Command::new(first_command.expect("Missing command."));

    let encryption = Encryption { config: &config };

    for (key, value) in config.configuration.iter() {
        command.env(key, encryption.decrypt(key, value)?);
    }

    for arg in sub_command {
        command.arg(arg);
    }

    if cfg!(unix) {
        #[cfg(unix)]
        command.exec();
        Ok(0)
    } else {
        let status = match command.status() {
            Ok(val) => val,
            Err(_) => return Err("Failed to start command, please verify that it exists."),
        };

        Ok(status.code().unwrap_or(0))
    }
}
