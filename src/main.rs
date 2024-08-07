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
    for input in TEST_INPUTS {
        parse_and_print(input)?;
    }

    Ok(())
}

const TEST_INPUTS: [&str; 4] = [
    r##"
        print x,
        set x = 10,
        let x = 1,, p89,
            8, "sreti", "hello", 90, a 90, 
            what ~,
            what ><<~-?>
    "##,
    r##"
        set x = 10,
        print x,
    "##,
    r##"
        set file = openfile "hello.txt",
        closefile = fn filename {
            print filename "closed",
            close filehandle
        },
        print x
    "##,
    r##"
        closefile = fn filename {
            filehandle = (gethandle filename),
        },
        print (closefile "hello.exas")
    "##,
];

fn parse_and_print(input: &'static str) -> Result<(), ParsingError> {
    println!("========== Source ===========");
    println!("{}", input);
    println!();

    println!("==========  AST   ===========");
    let mut parser = Parser::new(Token::lexer_with_extras(
        input,
        FileContext {
            filename: "test".to_string(),
            line: 1,
        },
    ));
    let ast = parser.parse()?;

    ast.print();
    println!();

    let mut program_ctx = ProgramContext {
        symbols: HashMap::new(),
        scope_stack: Vec::new(),
    };
    ast.build_context(&mut program_ctx, 0);

    println!("==========  Emit  ===========");
    let mut output = std::io::stdout();
    ast.check_and_emit(&mut output, &program_ctx, &mut vec![])
        .unwrap();

    println!("\n");
    Ok(())
}
