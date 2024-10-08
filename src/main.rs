#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use ast::{Ast, ProgramContext};
use clap::Parser as _;
use cli::{Cli, CliSubCommand, ExpansionArgs, Verbosity};
use errors::{CompileResult, ParsingError};
use lexer::{FileContext, Token};
use logos::Logos;
use parser::Parser;

mod ast;
mod cli;
mod errors;
mod layers;
mod lexer;
mod parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliSubCommand::Expand(ref build_args) => build(&cli, build_args),
        CliSubCommand::Run(ref build_args) => {
            build(&cli, build_args);
            todo!("And then run")
        }
    }
}

fn build(cli: &Cli, build_args: &ExpansionArgs) {
    let mut source = String::new();
    let compilation_result = match build_args.input_files.as_deref() {
        None => {
            io::stdin()
                .read_to_string(&mut source)
                .expect("Could not read stdin (might be caused by not enough memory)");

            compile("stdin", &source, &cli, &build_args)
        }
        Some([ref file]) => {
            fs::File::open(file)
                .unwrap()
                .read_to_string(&mut source)
                .expect("Could not read input file (might be caused by not enough memory)");

            compile(
                file.file_stem().unwrap().to_str().unwrap(),
                &source,
                &cli,
                &build_args,
            )
        }
        Some([..]) => panic!("Too many files"),
    };

    if let Err(error) = compilation_result {
        eprintln!("Compilation failed: {}", error);
        std::process::exit(1);
    }
}

fn compile<'source>(
    name: &'source str,
    source: &'source str,
    cli: &Cli,
    build_args: &ExpansionArgs,
) -> CompileResult<'source, ()> {
    let file_context = FileContext {
        filename: name.to_string(),
        line: 1,
        source,
    };

    // ================  Ast  ================= //
    let ast = get_ast(source, file_context.clone())?;

    if let Some(path) = build_args.ast.clone().or_else(|| {
        if build_args.all {
            Some(PathBuf::from(format!("{}.ast", name)))
        } else {
            None
        }
    }) {
        let mut ast_file = fs::File::create(&path)?;
        if cli.verbosity >= Verbosity::Info {
            println!("Emitting AST to {}", path.display());
        }
        write!(ast_file, "{:#?}", ast)?;
    }

    // ==========  Program Context  =========== //
    let mut program_ctx = ProgramContext {
        symbols: HashMap::new(),
        types: HashMap::new(),
        file_context: FileContext {
            line: 1,
            ..file_context
        },
    };
    ast.build_context(&mut program_ctx);
    if let Some(path) = build_args.symbols.clone().or_else(|| {
        if build_args.all {
            Some(PathBuf::from(format!("{}.sym", name)))
        } else {
            None
        }
    }) {
        let mut symbols_file = fs::File::create(&path)?;
        if cli.verbosity >= Verbosity::Info {
            println!("Emitting symbols to {}", path.display());
        }
        write!(symbols_file, "{:#?}", program_ctx.symbols)?;
    }

    // ===========  Expand CL -> AL  ========== //
    let result = match build_args.output {
        Some(ref path) => {
            let mut output_file = fs::File::create(path)?;
            if cli.verbosity >= Verbosity::Info {
                println!("Emitting to {}", path.display());
            }
            ast.expand_clayer(&mut output_file, &program_ctx)
        }
        None => {
            let mut output_file = io::stdout().lock();
            if cli.verbosity >= Verbosity::Info {
                println!("Emitting to stdout");
            }
            ast.expand_clayer(&mut output_file, &program_ctx)
        }
    };
    if let Err(e) = result {
        println!("\n{}", e);
        return Ok(());
    } else {
        println!("\nNo errors :)");
    }

    // ===========  Expand AL -> HL  ========== //
    // let result =

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
