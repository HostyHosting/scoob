mod config;
mod encryption;
mod manage;
mod start;

use crate::config::*;
use crate::encryption::*;
use crate::manage::*;
use crate::start::*;
use colored::Colorize;
use std::alloc::System;
use structopt::StructOpt;

#[global_allocator]
static A: System = System;

#[derive(Debug, StructOpt)]
#[structopt(name = "scoob", about = "A secrets management tool.")]
enum Opt {
    /// Manage a scoob configuration file
    Manage(Manage),

    /// Runs a command after loading scoob secrets into the environment
    Start(Start),
}

fn main() {
    if sodiumoxide::init().is_err() {
        return println!("{}", String::from("Was not able to initialize Sodium. Verify your installation of Scoob and try again.").red().bold());
    }

    let cli = Opt::from_args();

    let result = match &cli {
        Opt::Manage(c) => manage(&c),
        Opt::Start(c) => {
            let start_result = start(&c, false);

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
