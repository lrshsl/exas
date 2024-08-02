use std::collections::HashMap;

use lexer::Token;
use parser::{Parser, ParsingError};
use ast::{AstNode, ProgramContext, Ast};
use logos::Logos;

mod lexer;
mod parser;
mod ast;

fn compile(input: &'static str) -> Result<(), ParsingError> {
    let mut parser = Parser::new(Token::lexer(input));
    let ast = parser.parse()?;

    println!("========== Source ===========");
    println!("{}", input);
    println!();

    println!("==========  AST   ===========");
    ast.print();
    println!();

    let mut program_ctx = ProgramContext { symbols: HashMap::new() };
    ast.build_context(&mut program_ctx, 0);

    println!("==========  Emit  ===========");
    ast.emit(&program_ctx, &mut vec![0]);

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

