use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
enum SubCommand {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
struct Modify {
    /// Edit the provided configuration file
    #[structopt(short, long, about = "Test")]
    edit: bool,
    /// Create the configuration file
    #[structopt(short, long)]
    create: bool,
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct Start {
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    /// The sub-command that you wish to run
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "scoob", about = "TODO")]
enum Opt {
    /// Open a scoob configuration file for modification
    Modify(Modify),

    /// Runs a command after loading scoob configuration into the environment
    Start(Start),
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    configuration: HashMap<String, String>,
}

fn modify(cmd: &Modify) {
    let file = &cmd.file;
    let content = std::fs::read_to_string(file).expect("Could not read secrets file.");
    let deserialized_map: Config = serde_yaml::from_str(&content).expect("Do not break pls.");

    println!("{:?}", deserialized_map);
}

fn start(cmd: &Start) {
    let file = &cmd.file;
    let content = std::fs::read_to_string(file).expect("Could not read secrets file.");
    let config: Config = serde_yaml::from_str(&content).expect("Do not break pls.");

    println!("{:?}", config);

    let mut sub_command = match &cmd.sub_command {
        SubCommand::Other(values) => values.iter(),
    };

    if sub_command.len() < 1 {
        // TODO: Throw
        println!("Uh oh...")
    }

    let first_command = sub_command.next().unwrap();

    let mut command = Command::new(first_command);

    for key in config.configuration.keys() {
        command.env(
            key,
            config
                .configuration
                .get(key)
                .expect("Unexpected missing configuration"),
        );
    }

    for arg in sub_command {
        command.arg(arg);
    }

    command.status().expect("Failed to start");
}

fn main() {
    let cli = Opt::from_args();

    match &cli {
        Opt::Modify(c) => modify(&c),
        Opt::Start(c) => start(c),
    }

    // let content = std::fs::read_to_string(&opt).expect("could not read file");

    // println!("{:?}", );
}
