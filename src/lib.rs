pub mod cli;
pub mod layers;

mod ast;
mod errors;
mod lexer;
mod parser;

use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use ast::{Ast, ProgramContext};
use cli::{ExpansionArgs, Verbosity};
use errors::{CompileResult, ParsingError};
use lexer::{FileContext, Token};
use logos::Logos;
use parser::Parser;

pub fn build(build_args: &ExpansionArgs, verbosity: Verbosity) {
    let (source_name, source) = match build_args.input_files.as_deref() {
        Some([ref filename]) => (
            filename.file_stem().unwrap().to_str().unwrap(),
            fs::read_to_string(filename)
                .expect("Could not read input file (might be caused by not enough memory)"),
        ),
        None => (
            "stdin",
            io::read_to_string(io::stdin().lock())
                .expect("Could not read stdin (might be caused by not enough memory)"),
        ),
        Some([..]) => panic!("Too many files"),
    };

    let default_ast_output = PathBuf::from(format!("{source_name}.ast"));
    let ast_output = build_args.ast.as_ref().or_else(|| {
        if build_args.all {
            Some(&default_ast_output)
        } else {
            None
        }
    });

    let default_symbols_output = PathBuf::from(format!("{source_name}.sym"));
    let symbols_output = build_args.symbols.as_ref().or_else(|| {
        if build_args.all {
            Some(&default_symbols_output)
        } else {
            None
        }
    });

    let compilation_result = match build_args.output.as_ref() {
        None => {
            let mut output = io::stdout().lock();

            compile(
                source_name,
                &source,
                &mut output,
                ast_output,
                symbols_output,
                verbosity,
            )
        }
        Some(ref output_file) => {
            let mut output_file =
                fs::File::create(output_file).expect("Could not open output file");

            compile(
                source_name,
                &source,
                &mut output_file,
                ast_output,
                symbols_output,
                verbosity,
            )
        }
    };

    if let Err(error) = compilation_result {
        eprintln!("Compilation failed: {}", error);
        std::process::exit(1);
    }
}

pub fn compile<'source>(
    name: &'source str,
    source: &'source str,
    output: &mut impl io::Write,
    ast_output: Option<&PathBuf>,
    symbols_output: Option<&PathBuf>,
    verbosity: Verbosity,
) -> CompileResult<'source, ()> {
    let file_context = FileContext {
        filename: name.to_string(),
        line: 1,
        source,
    };

    // ================  Ast  ================= //
    let ast = get_ast(source, file_context.clone())?;

    if let Some(path) = ast_output {
        let mut ast_file = fs::File::create(&path)?;
        if verbosity >= Verbosity::Info {
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
    if let Some(path) = symbols_output {
        let mut symbols_file = fs::File::create(&path)?;
        if verbosity >= Verbosity::Info {
            println!("Emitting symbols to {}", path.display());
        }
        write!(symbols_file, "{:#?}", program_ctx.symbols)?;
    }

    // ===========  Expand CL -> AL  ========== //
    let result = ast.expand_clayer(output, &program_ctx);
    // let result = match output {
    //     Some(ref path) => {
    //         let mut output_file = fs::File::create(path)?;
    //         if verbosity >= Verbosity::Info {
    //             println!("Emitting to {}", path.display());
    //         }
    //         ast.expand_clayer(&mut output_file, &program_ctx)
    //     }
    //     None => {
    //         let mut output_file = io::stdout().lock();
    //         if verbosity >= Verbosity::Info {
    //             println!("Emitting to stdout");
    //         }
    //         ast.expand_clayer(&mut output_file, &program_ctx)
    //     }
    // };
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
