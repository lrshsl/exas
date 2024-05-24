use lexer::Token;
use parser::{Parser, ParsingError};
use logos::Logos;

mod lexer;
mod parser;
mod ast;

fn main() -> Result<(), ParsingError> {
    let input = "
        print x
        set x = 10
        ";
    let mut parser = Parser::new(Token::lexer(input));
    let ast = parser.parse()?;
    ast.print();

    println!("\n");
    Ok(())
}

