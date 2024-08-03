grammar bnf;

ast
	: listcontent EOF
	;

list
	: '{' listcontent '}'  // Immutable
	| '[' listcontent ']'  // Mutable
	;

listcontent
	: (Annex? expr? ',')* expr? Annex?
	;

expr
	: lambda
	| IntLiteral
	| StringLiteral
	| Id '<-' expr		// Move
	| Id '=' expr		// Label
	| fnCall
	| '(' expr ')'
	| list
	;

lambda
	: 'fn' param* list
	;

fnCall
	: Id argument*
	;

argument
	: anyToken
	| list
	| '(' expr ')'
	;

param
	: anyToken
	| '[' Id? (':' Id)? ']'
	;

anyToken
	: Id
	| IntLiteral
	| StringLiteral
	| '!'  | '#' | '$' | '%' | '&' | '\''
	| '*'  | '+' | '-' | '.' | '/' | ':'
	| ';'  | '<' | '=' | '>' | '?' | '@'
	| '\\' | '^' | '_' | '`' | '|' | '~'
	;

//---|> Lexer rules <|---//

CommentOrDoc
	: (LineComment | BlockComment | InlineDoc | BlockDoc) -> channel(HIDDEN)
	;

LineComment
	: '//' ~[\n\r]*
	;

BlockComment
	: '/..' (CommentOrDoc | ~[.])*? '../'
	;

// Tag
InlineDoc
	: '\'' ~[\n\r]* '\'' -> channel(HIDDEN)
	;

BlockDoc
	: '///' ~[\n\r]* -> channel(HIDDEN)
	;

Annex
	: '|' ~[\n\r]*
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

WS 
	: [ \t\r\n]+ -> skip
	;

