mod config;
mod encryption;
mod modify;
mod start;

use crate::config::*;
use crate::encryption::*;
use crate::modify::*;
use crate::start::*;
use colored::Colorize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
pub struct Modify {
    /// Edit the provided configuration file
    #[structopt(short, long)]
    edit: bool,
    /// Create the configuration file
    #[structopt(short, long)]
    create: bool,
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
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

#[derive(Debug, StructOpt)]
#[structopt(name = "scoob", about = "A secrets management tool.")]
enum Opt {
    /// Open a scoob configuration file for modification
    Modify(Modify),

    /// Runs a command after loading scoob configuration into the environment
    Start(Start),
}

fn main() {
    sodiumoxide::init().expect("Was not able to initialize Libsodium.");

    let cli = Opt::from_args();

    let result = match &cli {
        Opt::Modify(c) => modify(&c),
        Opt::Start(c) => {
            let start_result = start(c);

            match start_result {
                Ok(status) => std::process::exit(match status.code() {
                    Some(code) => code,
                    None => 0,
                }),
                Err(err) => Err(err),
            }
        }
    };

    match result {
        Ok(_) => (),
        Err(message) => println!("{}", String::from(message).red().bold()),
    };
}
