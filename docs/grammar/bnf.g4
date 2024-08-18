grammar bnf;

ast
	: listcontent EOF
	;

list
	: '{' listcontent '}'  // Const
	| '[' listcontent ']'  // Mutable
	;

listcontent
	//: (Annex? expr? ',')* expr? Annex?
	: expr? (',' expr?)*
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
	| '\\' | '^' | '_' | '`' | '~'
	;

//---|> Lexer rules <|---//

CommentOrDoc
	: (Comment | Doc) -> channel(HIDDEN)
	;

Doc
	: '||' ~[|\n\r]* '||'?
	;

Comment
	: '|' ~[|\n\r]* '|'?
	;

//BlockComment
//	: '|..' (CommentOrDoc | ~[.])*? '..|'
//	;

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

