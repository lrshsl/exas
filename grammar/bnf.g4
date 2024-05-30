grammar bnf;		

ast
    : line* EOF
    ;

line
    : stmt NL
    | NL
    ;

stmt
    : expr
    | 'print' expr
    | Ident '<-' expr
    | expr '->' Ident
    ;

primaryExpression
    : IntLiteral
    | StringLiteral
    | Ident
    | '(' expr ')'
    ;

expr
    : primaryExpression
    | expr ('*'|'/') expr
    | expr ('+'|'-') expr
    | Ident '<-' expr
    | expr '->' Ident
    | expr ('<'|'>'|'=='|'!='|'<='|'>=') expr
    | expr '&&' expr
    | expr '||' expr
    ;


//---|> Lexer rules <|---//

CommentOrDoc
    : (LineComment | BlockComment | BlockDoc) -> channel(HIDDEN)
    ;

LineComment
    : ('//' (~[/!] | '//') ~[\r\n]* | '//') -> channel (HIDDEN)
    ;

BlockComment
    : (
        '/*' (~[*!] | '**' | BlockComment | BlockDoc) (BlockComment | BlockDoc | ~[*])*? '*/'
        | '/**/'
        | '/***/'
    ) -> channel(HIDDEN)
    ;

BlockDoc
    : '///' (~[/] ~[\n\r]*)? -> channel(HIDDEN)
    ;


// String Literals

StringLiteral
    : RawStringLiteral
    | EscStringLiteral
    ;

RawStringLiteral
    : '\'' .*? '\''
    ;

EscStringLiteral
    : '"' .*? '"'
    ;


// Numeric Literals

IntLiteral
    : DIGIT+
    ;

Ident
    : ALPHA (ALPHA|DIGIT)*
    ;


// Dead simple matches IN_CAPS

fragment ALPHA
    : [A-Za-z]
    ;

fragment DIGIT
    : [0-9]
    ;

NL  : [\r\n]+ ;

WS  : [ \t]+ -> skip ;

