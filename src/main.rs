use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
enum Command {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "scoob", about = "TODO")]
enum Opt {
    /// Open a scoob configuration file for modification
    Modify {
        /// Edit the provided configuration file
        #[structopt(short, long, about = "Test")]
        edit: bool,
        /// Create the configuration file
        #[structopt(short, long)]
        create: bool,
        /// Path to the scoob configuration file
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },

    /// Runs a command after loading scoob configuration into the environment
    Start {
        /// Path to the scoob configuration file
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        /// The sub-command that you wish to run
        #[structopt(subcommand)]
        command: Command,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
