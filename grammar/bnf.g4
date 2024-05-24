grammar bnf;		

ast
    : stmt* EOF
    ;

stmt
    : expr NL
    | 'print' expr NL
    | ident '=' expr NL
    ;

expr
    :	expr ('*'|'/') expr
    |	expr ('+'|'-') expr
    |	INT
    |	'(' expr ')'
    ;

ident
    : (ALPHA | '_') (ALPHANUM | '_')*
    ;


// Lexer rules

ALPHANUM
    : ALPHA
    | INT
    ;

ALPHA
    : [A-Za-z]
    ;

INT
    : [0-9]+
    ;

NL
    : [\r\n]+
    ;

WS
    : [ \t]+ -> skip
    ;

