use std::path::PathBuf;

use clap::{arg, command, Args, Parser as ArgParser, Subcommand};

#[derive(ArgParser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: CliSubCommand,

    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u8).range(0..4))]
    pub verbosity: u8,
}

#[derive(Subcommand)]
pub enum CliSubCommand {
    Build(BuildArgs),
    Run(BuildArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    /// Input source files. Can be omitted to read from stdin
    pub input_files: Option<Vec<PathBuf>>,

    /// Write output to a file. Can be omitted to write to stdout
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Emit <out>.ast and <out>.sym files
    #[arg(short, long)]
    pub all: bool,

    /// Emit the ast to a file
    #[arg(long, value_name = "FILE")]
    pub ast: Option<PathBuf>,

    /// Emit the entire symbol table to a file
    #[arg(long, value_name = "FILE")]
    pub symbols: Option<PathBuf>,
}

#[derive(Args)]
pub struct RunArgs {}
