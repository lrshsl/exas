#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use clap::Parser as _;
use eas::cli::{Cli, CliSubCommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliSubCommand::Expand(ref build_args) => eas::build(build_args, cli.verbosity),
        CliSubCommand::Run(ref build_args) => {
            eas::build(build_args, cli.verbosity);
            todo!("And then run")
        }
    }
}
