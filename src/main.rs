use std::collections::HashMap;

use lexer::Token;
use parser::{Parser, ParsingError};
use ast::{AstNode, ProgramContext, Ast};
use logos::Logos;

mod lexer;
mod parser;
mod ast;

fn main() -> Result<(), ParsingError> {
    let input = r##"
        print x,
        set x = 10,
        let x = 1,, p89,
            8, "sreti", "hello", 90, a 90, 
            what ~,
            what = ~-?>
        "##;
    let mut parser = Parser::new(Token::lexer(input));
    let ast = parser.parse()?;
    ast.print();

    let mut program_ctx = ProgramContext { symbols: HashMap::new() };
    ast.build_context(&mut program_ctx, 0);
    ast.emit(&program_ctx, &mut vec![0]);

    println!();
    Ok(())
}

