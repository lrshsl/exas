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
    // let filename = std::env::args().nth(1).expect("no file provided");
    // let input = std::fs::read_to_string(&filename).expect("failed to read file");
    let (name, input) = TEST_INPUTS[0];
    parse_and_print(&name, &input)?;

    Ok(())
}

const TEST_INPUTS: [(&str, &str); 4] = [
    (
        "function_calls.exas",
        r##"
        print = fn x {},
        set = fn name = val {},
        what = fn {},
        p89 = fn {},
        a = fn {},

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
        print = fn {},
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
    println!("<<<{}>>>", input);

    println!("\n\n==========  AST   ===========\n");
    let file_context = FileContext {
        file: name.to_string(),
        source: input,
        line: 1,
    };
    let mut parser = Parser::new(Token::lexer_with_extras(input, file_context.clone()));
    let ast = parser.parse()?;

    println!("{:#?}", ast);

    println!("\n\n==========  Program Context  ===========\n");
    let mut program_ctx = ProgramContext {
        symbols: HashMap::new(),
        file_context: FileContext {
            line: 1,
            ..file_context
        },
    };
    ast.build_context(&mut program_ctx);
    println!("{:#?}", program_ctx);

    println!("\n\n==========  Emit  ===========");
    let mut output_file = std::io::stdout().lock();
    let result = ast.check_and_emit(&mut output_file, &program_ctx);

    println!("\n\n==========  Output  ===========");
    if let Err(e) = result {
        println!("\n{}", e);
    } else {
        println!("\nNo errors :)");
    }

    println!("\n");
    Ok(())
}
