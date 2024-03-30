#[allow(unused)]
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub mod token;
pub mod lexer;

fn main() {
	let input = "let x = 5";
	let mut lexer = Lexer::new(input.to_string());
	let mut v: Vec<Token> = Vec::new();
	
	let mut token = lexer.next_token();
	while token.token_type != TokenKind::EOF {
		v.push(token);
		token = lexer.next_token();
	}

	v.push(token);

	println!("{:#?}", v);
}
