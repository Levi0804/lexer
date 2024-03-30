pub mod token;
pub mod lexer;

#[cfg(test)]
mod tests {
	use crate::token::{Token, TokenKind};
	use crate::lexer::Lexer;

	#[test]
	fn test_lexer() {
		let input = "=+(){},; ";

		let mut lexer = Lexer::new(input.to_string());
		let mut v: Vec<Token> = Vec::new();
		
		let mut token = lexer.next_token();
		while token.token_type != TokenKind::EOF {
			v.push(token);
			token = lexer.next_token();
		}
	
		v.push(token);

		assert_eq!(
			v, vec![
				Token { token_type: TokenKind::ASSIGN, literal: String::from("=") },
				Token { token_type: TokenKind::PLUS, literal: String::from("+") },
				Token { token_type: TokenKind::LPAREN, literal: String::from("(") },
				Token { token_type: TokenKind::RPAREN, literal: String::from(")") },
				Token { token_type: TokenKind::LBRACE, literal: String::from("{") },
				Token { token_type: TokenKind::RBRACE, literal: String::from("}") },
				Token { token_type: TokenKind::COMMA, literal: String::from(",") },
				Token { token_type: TokenKind::SEMICOLON, literal: String::from(";") },
				Token { token_type: TokenKind::EOF, literal: String::from("") },
			]
		);
	}
}