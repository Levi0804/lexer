#[derive(Debug, PartialEq)]
pub struct Token {
	pub token_type: TokenKind,
	pub literal: String,
}

// we will feed string to the lexer for it for perform the lexicaly analysis
#[derive(Debug, PartialEq)]
pub enum TokenKind {
	ILLEGAL ,
	EOF,
	// Identifiers + literals
	IDENT, 	 // add, foobar, x, y, ...
	INT, 	 // 729
	// Operators
	ASSIGN,
	PLUS,
	// Delimiters
	COMMA,
	SEMICOLON,
	LPAREN,
	RPAREN,
	LBRACE,
	RBRACE,
	// Keywords
	FUNCTION,
	LET,
}