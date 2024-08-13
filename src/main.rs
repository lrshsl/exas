#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use std::collections::HashMap;

use ast::{AstNode, ProgramContext};
use lexer::{FileContext, Token};
use logos::Logos;
use parser::Parser;
use parsing_error::ParsingError;

mod ast;
mod lexer;
mod parser;
mod parsing_error;

fn main() -> Result<(), ParsingError> {
    for (name, input) in TEST_INPUTS {
        parse_and_print(name, input)?;
    }

    Ok(())
}

const TEST_INPUTS: [(&str, &str); 4] = [
    (
        "function_calls.exas",
        r##"
        print = fn x {},
        set = fn name = val {},
        what = fn {},

        print x,
        set x = 10,
        print x = 1,, p89,
            8, "sreti", "hello", 90, a 90, 
            what ~,
            what ><<~-?>
    "##,
    ),
    (
        "small_test.exas",
        r##"
        set x = 10,
        print x,
    "##,
    ),
    (
        "small_test.exas",
        r##"
        set file = openfile "hello.txt",
        closefile = fn filename {
            print filename "closed",
            close filehandle
        },
        print x
    "##,
    ),
    (
        "subexpressions.exas",
        r##"
        gethandle = fn filename {},
        closefile = fn filename {
            filehandle = (gethandle filename),
        },
        print (closefile "hello.exas")
    "##,
    ),
];

fn parse_and_print(name: &'static str, input: &'static str) -> Result<(), ParsingError> {
    println!("========== Source ===========");
    println!("{}", input);
    println!();

    let file_context = FileContext {
        file: name.to_string(),
        source: input,
        line: 1,
    };

    println!("==========  AST   ===========");
    let mut parser = Parser::new(Token::lexer_with_extras(input, file_context.clone()));
    let ast = parser.parse()?;

    println!("{}", ast);
    println!();

    let mut program_ctx = ProgramContext {
        symbols: HashMap::new(),
        file_context: FileContext {
            line: 1,
            ..file_context
        },
    };
    ast.build_context(&mut program_ctx, 0);

    println!("==========  Emit  ===========");
    let mut output = std::io::stdout().lock();
    if let Err(e) = ast.check_and_emit(&mut output, &program_ctx, &mut vec![]) {
        println!("{}", e);
    }

    println!("\n");
    Ok(())
}
