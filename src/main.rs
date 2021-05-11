mod config;
mod encryption;
mod start;
mod modify;

use crate::config::*;
use crate::modify::*;
use crate::start::*;
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
pub struct Start {
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

fn main() {
    sodiumoxide::init().unwrap();

    let cli = Opt::from_args();

    // TODO: Take result and do shit with it.
    match &cli {
        Opt::Modify(c) => modify(&c),
        Opt::Start(c) => start(c),
    }.unwrap();
}
