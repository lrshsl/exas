use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext<'source> {
    pub filename: String,
    pub source:   &'source str,
    pub line:     usize,
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
#[logos(skip r"[ \t\r\f]+")]
pub enum Token {
    #[regex(r"[_[:alpha:]][_[:alpha:][:digit:]]*")]
    Ident,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u32>().expect("Wrong"))]
    Int(u32),

    #[token(r"fn")]
    KeywordFn,

    #[token(r"type")]
    KeywordType,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u\p{hexdigit}{4})*""#)]
    String,

    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        Skip
    })]
    Newline,

    #[regex(r"\|\|[^\n]*\|\|", logos::skip, priority = 3)]
    DocComment,

    #[regex(r"\|[^\n|]*", logos::skip, priority = 2)]
    Comment,

    #[regex(r".", |lex| lex.slice().chars().next().expect("Wrong: empty symbol"), priority = 1)]
    Symbol(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_print() {
        let source = r##"
            set x = 3,
            set y = 4,
            print x,
            "##;
        let mut lex = Token::lexer_with_extras(source, FileContext {
            filename: "test_set_print".to_string(),
            source,
            line: 1,
        });
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "set");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('='))));
        assert_eq!(lex.next(), Some(Ok(Token::Int(3))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('='))));
        assert_eq!(lex.next(), Some(Ok(Token::Int(4))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "print");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn symbols() {
        let source = r##"
            a - < > *  / ,.? :,
            "##;
        let mut lex = Token::lexer_with_extras(source, FileContext {
            filename: "test_symbols".to_string(),
            source,
            line: 1,
        });
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('-'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('<'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('>'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('*'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('/'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('.'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('?'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(':'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn all_symbols() {
        let source = r##"
            echo "hello",
            is .. x > y or .. < y ?
                : "hello",

            let x -> y,
            "##;
        let mut lex = Token::lexer_with_extras(source, FileContext {
            filename: "test3".to_string(),
            source,
            line: 1,
        });
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "echo");
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.slice(), "\"hello\"");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "is");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('.'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('.'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('>'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "or");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('.'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('.'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('<'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('?'))));

        assert_eq!(lex.next(), Some(Ok(Token::Symbol(':'))));
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.slice(), "\"hello\"");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));

        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "let");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('-'))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('>'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol(','))));
    }

    #[test]
    fn identifiers() {
        let source = r##"
            these-are-7-symbols

            this_is_all_just_one
            This_asWell42?
            "##;
        let mut lex = Token::lexer_with_extras(source, FileContext {
            filename: "test4".to_string(),
            source,
            line: 1,
        });
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('-'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('-'))));
        assert_eq!(lex.next(), Some(Ok(Token::Int(7))));
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('-'))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "this_is_all_just_one");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "This_asWell42");
        assert_eq!(lex.next(), Some(Ok(Token::Symbol('?'))));
        assert_eq!(lex.next(), None);
    }
}
