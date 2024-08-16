#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::{Parser as ArgParser, Subcommand};

use ast::{Ast, ProgramContext};
use lexer::{FileContext, Token};
use logos::Logos;
use parser::Parser;
use parsing_error::ParsingError;

mod ast;
mod lexer;
mod parser;
mod parsing_error;

#[derive(ArgParser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input source files. Can be omitted to read from stdin
    input_files: Option<Vec<PathBuf>>,

    /// Write output to a file. Can be omitted to write to stdout
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u8).range(0..4))]
    verbosity: u8,

    /// Emit <out>.ast and <out>.sym files
    #[arg(short, long)]
    all: bool,

    /// Emit the ast to a file
    #[arg(long, value_name = "FILE")]
    ast: Option<PathBuf>,

    /// Emit the entire symbol table to a file
    #[arg(long, value_name = "FILE")]
    symbols: Option<PathBuf>,

    /// Subcommands
    #[command(subcommand)]
    command: CliSubCommand,
}

#[derive(Subcommand)]
enum CliSubCommand {
    Build,
    Run,
}

fn main() {
    let cli = Cli::parse();

    let mut source = String::new();
    match cli.input_files.as_deref() {
        None => {
            io::stdin().read_to_string(&mut source).unwrap();

            compile("stdin", &source, &cli)
        }
        Some([ref file]) => {
            fs::File::open(file)
                .unwrap()
                .read_to_string(&mut source)
                .unwrap();

            compile(file.to_str().unwrap(), &source, &cli)
        }
        Some([..]) => panic!("Too many files"),
    }
    .unwrap()
}

pub type CompileResult<'source, T> = Result<T, CompileError<'source>>;

#[derive(Debug)]
pub enum CompileError<'source> {
    ParsingError(ParsingError<'source>),
    IoError(io::Error),
}

impl<'source> From<ParsingError<'source>> for CompileError<'source> {
    fn from(e: ParsingError<'source>) -> Self {
        CompileError::ParsingError(e)
    }
}

impl From<io::Error> for CompileError<'_> {
    fn from(e: io::Error) -> Self {
        CompileError::IoError(e)
    }
}

fn compile<'source>(
    name: &'source str,
    source: &'source str,
    cli: &Cli,
) -> CompileResult<'source, ()> {
    let file_context = FileContext {
        filename: name.to_string(),
        line: 1,
        source,
    };

    // ================  Ast  ================= //
    let ast = get_ast(source, file_context.clone())?;

    if let Some(path) = &cli.ast.clone().or_else(|| {
        if cli.all {
            Some(PathBuf::from(format!("{}.ast", name)))
        } else {
            None
        }
    }) {
        let mut ast_file = fs::File::create(path)?;
        write!(ast_file, "{:#?}", ast)?;
    }

    // ==========  Program Context  =========== //
    let mut program_ctx = ProgramContext {
        symbols: HashMap::new(),
        file_context: FileContext {
            line: 1,
            ..file_context
        },
    };
    ast.build_context(&mut program_ctx);
    if let Some(path) = &cli.symbols.clone().or_else(|| {
        if cli.all {
            Some(PathBuf::from(format!("{}.sym", name)))
        } else {
            None
        }
    }) {
        let mut symbols_file = fs::File::create(path)?;
        write!(symbols_file, "{:#?}", program_ctx)?;
    }

    // ==============  Expand  ================ //
    let result = match cli.output {
        Some(ref path) => {
            let mut output_file = fs::File::create(path)?;
            ast.check_and_emit(&mut output_file, &program_ctx)
        }
        None => {
            let mut output_file = io::stdout().lock();
            ast.check_and_emit(&mut output_file, &program_ctx)
        }
    };

    // ==========  Compiler Output  =========== //
    if let Err(e) = result {
        println!("\n{}", e);
    } else {
        println!("\nNo errors :)");
    }

    Ok(())
}

fn get_ast<'source>(
    input: &'source str,
    file_context: FileContext<'source>,
) -> Result<Ast<'source>, ParsingError<'source>> {
    Parser::new(Token::lexer_with_extras(input, file_context)).parse()
}
