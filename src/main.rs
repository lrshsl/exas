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

fn compile(input: &'static str) -> Result<(), ParsingError> {
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
    };
    ast.build_context(&mut program_ctx, 0);

    println!("==========  Emit  ===========");
    ast.check_and_emit(&program_ctx, &mut vec![0]);

    println!("\n");
    Ok(())
}

fn main() -> Result<(), ParsingError> {
    let inputs = [
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

    for input in inputs {
        compile(input)?;
    }

    Ok(())
}
