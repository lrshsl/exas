use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[\p{whitespace}]+")] // Ignore whitespace
pub enum Token<'source> {
    #[regex("[[:alpha:]][[:alpha:][:digit:]]*")]
    Ident,

    #[regex("[0-9]+")]
    Int,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u\p{hexdigit}{4})*""#)]
    String,

    #[regex(r###"[^0-9a-zA-Z\p{whitespace}]"###)]
    Symbol(&'source str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_print() {
        let mut lex = Token::lexer(
            "
            set x = 3,
            set y = 4,
            print x,
            ",
        );
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "set");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("="))));
        assert_eq!(lex.next(), Some(Ok(Token::Int)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("="))));
        assert_eq!(lex.next(), Some(Ok(Token::Int)));
        assert_eq!(lex.slice(), "4");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "print");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn symbols() {
        let mut lex = Token::lexer(
            r#"
            a - < > *  / ,.? :,
            "#
            );
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("-"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("<"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(">"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("*"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("/"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("."))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("?"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(":"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn all_symbols() {
        let mut lex = Token::lexer(
            r#"
            echo "hello",
            is .. x > y || .. < y ?
                : "hello",

            let x -> y,
            "#,
        );
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "echo");
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.slice(), "\"hello\"");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "is");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("."))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("."))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(">"))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("|"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("|"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("."))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("."))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("<"))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("?"))));

        assert_eq!(lex.next(), Some(Ok(Token::Symbol(":"))));
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.slice(), "\"hello\"");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "let");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("-"))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(">"))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));
    }
}
