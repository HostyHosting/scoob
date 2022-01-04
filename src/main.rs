mod config;
mod encryption;
mod file;
mod generate_keys;
mod manage;
mod start;

use colored::Colorize;
use std::alloc::System;
use structopt::StructOpt;

#[global_allocator]
static A: System = System;

#[derive(Debug, StructOpt)]
#[structopt(name = "scoob", about = "A secrets management tool.")]
enum Opt {
    /// Manage a scoob configuration file
    Manage(crate::manage::Manage),

    /// Runs a command after loading scoob secrets into the environment
    Start(crate::start::Start),

    /// Utilities for encrypting files
    File(crate::file::File),

    /// Generate a keypair that can be used as encryption keys
    GenerateKeys(crate::generate_keys::GenerateKeys),
}

fn main() {
    if sodiumoxide::init().is_err() {
        return println!("{}", String::from("Was not able to initialize Sodium. Verify your installation of Scoob and try again.").red().bold());
    }

    // Load the .env file into the current environment:
    dotenv::dotenv().ok();

    let cli = Opt::from_args();

    let result = match &cli {
        Opt::GenerateKeys(c) => crate::generate_keys::generate_keys(c),
        Opt::Manage(c) => crate::manage::manage(c),
        Opt::File(c) => crate::file::file(c),
        Opt::Start(c) => {
            let start_result = crate::start::start(c);

            match start_result {
                Ok(status) => std::process::exit(status),
                Err(err) => Err(err),
            }
        }
    };

    match result {
        Ok(_) => (),
        Err(message) => println!("{}", String::from(message).red().bold()),
    };
}
