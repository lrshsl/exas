use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext {
    pub filename: String,
    pub line: usize,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = FileContext)]
#[logos(skip r"[ \t\r\f]+")] // Ignore whitespace
pub enum Token<'source> {
    #[regex(r"[[:alpha:]][[:alpha:][:digit:]]*")]
    Ident,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().expect("Wrong"))]
    Int(i32),

    #[token(r"fn")]
    KeywordFn,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u\p{hexdigit}{4})*""#)]
    String,

    #[regex(r###"[^0-9a-zA-Z\p{whitespace}]"###)]
    Symbol(&'source str),

    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        Skip
    })]
    Newline,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_print() {
        let mut lex = Token::lexer_with_extras(
            "
            set x = 3,
            set y = 4,
            print x,
            ",
            FileContext {
                filename: "test1".to_string(),
                line: 1,
            },
        );
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "set");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("="))));
        assert_eq!(lex.next(), Some(Ok(Token::Int(3))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(","))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol("="))));
        assert_eq!(lex.next(), Some(Ok(Token::Int(4))));
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
        let mut lex = Token::lexer_with_extras(
            r#"
            a - < > *  / ,.? :,
            "#,
            FileContext {
                filename: "test2".to_string(),
                line: 1,
                column: 1,
            },
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
        let mut lex = Token::lexer_with_extras(
            r#"
            echo "hello",
            is .. x > y || .. < y ?
                : "hello",

            let x -> y,
            "#,
            FileContext {
                filename: "test3".to_string(),
                line: 1,
                column: 1,
            },
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
