use crate::config::Config;
use crate::encryption::Encryption;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
pub struct Start {
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    /// The sub-command that you wish to run
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

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

    // Only attempt to exec on unix, and when we're not running tests
    if cfg!(unix) && !cfg!(test) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_secrets_path() -> PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("test");
        path.push("secrets.yml");
        path
    }

    #[test]
    fn test_start_no_command() {
        assert!(start(&Start {
            file: get_secrets_path(),
            sub_command: SubCommand::Other(vec![])
        })
        .is_err());
    }

    #[test]
    fn test_start_invalid_command() {
        assert!(start(&Start {
            file: get_secrets_path(),
            sub_command: SubCommand::Other(vec!["command_does_not_exist".to_string()])
        })
        .is_err());
    }

    #[test]
    fn test_start_print() {
        assert_eq!(
            start(&Start {
                file: get_secrets_path(),
                sub_command: SubCommand::Other(vec![
                    "sh".to_string(),
                    "./test/compare.sh".to_string()
                ])
            })
            .unwrap(),
            0
        );
    }
}
