#[allow(unused)]
pub struct Token {
	pub token_type: TokenKind,
	pub literal: String,
}

// we will feed string to the lexer for it for perform the lexicaly analysis
pub enum TokenKind {
	ILLEGAL(String),
	EOF(String),
	// Identifiers + literals
	IDENT(String), 				// add, foobar, x, y, ...
	INT(String), 				// 729
	// Operators
	ASSIGN(String),
	PLUS(String),
	// Delimiters
	COMMA(String),
	SEMICOLON(String),
	LPAREN(String),
	RPAREN(String),
	LBRACE(String),
	RBRACE(String),
	// Keywords
	FUNCTION(String),
	LET(String),
}