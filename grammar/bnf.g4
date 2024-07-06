grammar bnf;		

ast
    : listcontent EOF
    ;

list
    : '{' listcontent '}'  // Immutable
    | '[' listcontent ']'  // Mutable
    ;

listcontent
    : expr (',' expr?)*
    ;

expr
    : primaryExpression
    | expr ('*'|'/') expr
    | expr ('+'|'-') expr
    | expr ('<'|'>'|'=='|'!='|'<='|'>=') expr
    | expr '&&' expr
    | expr '||' expr
    ;

primaryExpression
    : fn
	 | fnCall
    | IntLiteral
    | StringLiteral
    | Id
    | '(' expr ')'
    ;

fn
    : '(' Id* ')' list
    ;

fnCall
	: Id ( anyToken | list | '(' expr ')' | CommentOrDoc )*
	;

//---|> Lexer rules <|---//

anyToken
	: Id
	| IntLiteral
	| StringLiteral
	| '!'  | '#' | '$' | '%' | '&' | '\''
	| '*'  | '+' | '-' | '.' | '/' | ':'
	| ';'  | '<' | '=' | '>' | '?' | '@'
	| '\\' | '^' | '_' | '`' | '|' | '~'
	;

CommentOrDoc
    : (LineComment | BlockComment | InlineDoc | BlockDoc) -> channel(HIDDEN)
    ;

LineComment
    : '//' ~[\n\r]*
    ;

BlockComment
    : '/..' (CommentOrDoc | ~[.])*? '../'
    ;

InlineDoc
    : '\'' ~[\n\r]* '\'' -> channel(HIDDEN)
    ;

BlockDoc
    : '///' ~[\n\r]* -> channel(HIDDEN)
    ;

Reg
    : 'r' [0-9]+
    ;


// String Literals

StringLiteral
    : RawStringLiteral
    | FormatString
    ;

RawStringLiteral
    : '"' .*? '"'
    ;

FormatString
    : 'f' '"' .*? '"'
    ;


// Numeric Literals

IntLiteral
    : DIGIT+
    ;

Id
    : ALPHA (ALPHA | DIGIT)*
    ;


// Fragments

fragment ALPHA
    : [A-Za-z]
    ;

fragment DIGIT
    : [0-9]
    ;

WS  : [ \t\r\n]+ -> skip ;

