use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore whitespace
pub enum Token {
    #[token("set")]
    Set,
    #[token("print")]
    Print,

    #[regex(r"\n")]
    Newline,

    #[token("=")]
    Eq,

    #[regex("[a-zA-Z]+")]
    Ident,

    #[regex("[0-9]+")]
    Int,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_print() {
        let mut lex = Token::lexer(
            "
            set x = 3
            set y = 4
            print x
            ",
        );
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Set)));
        assert_eq!(lex.slice(), "set");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.slice(), "=");
        assert_eq!(lex.next(), Some(Ok(Token::Int)));
        assert_eq!(lex.slice(), "3");
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));

        assert_eq!(lex.next(), Some(Ok(Token::Set)));
        assert_eq!(lex.slice(), "set");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.slice(), "=");
        assert_eq!(lex.next(), Some(Ok(Token::Int)));
        assert_eq!(lex.slice(), "4");
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));

        assert_eq!(lex.next(), Some(Ok(Token::Print)));
        assert_eq!(lex.slice(), "print");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), None);
    }
}
