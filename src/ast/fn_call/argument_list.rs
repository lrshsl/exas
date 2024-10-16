use super::*;
use crate::assert_token_matches;

pub type ArgumentList<'source> = Vec<RawToken<'source>>;

impl<'source> Parsable<'source> for ArgumentList<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<ArgumentList<'source>, ParsingError<'source>> {
        let mut args = vec![];
        while let Some(Ok(token)) = parser.current_token {
            match token {
                Token::Symbol(',') => {
                    parser.advance();
                    break;
                }
                Token::Symbol('(') => {
                    parser.advance(); // Consume the '('
                    args.push(RawToken::Expr(Expr::parse(parser)?)); // Parse the expression
                    assert_token_matches!(
                        parser.current_token,
                        Token::Symbol(')'),
                        parser.lexer.extras
                    );
                    parser.advance(); // Consume the ')'
                }
                token => {
                    args.push(RawToken::from_token(token, parser.current_slice));
                    parser.advance();
                } // TODO: Some other tokens are not allowed here?
            }
        }
        Ok(args)
    }
}
