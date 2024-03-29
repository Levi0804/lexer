use crate::token::{Token, TokenKind};

#[allow(unused)]
#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,         // current position in input (points to current char)
    read_position: usize,    // current reading position in input (after current char)
    ch: u8,                  // current char in examination
}

#[allow(unused)]
impl Lexer {
    fn new(input: String) -> Self {
        let mut l = Self { input, position: 0, read_position: 0, ch: 0 };
        l.read_char();
        return l;
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

	fn next_token(&mut self) -> Token  {
		let token: Token = match self.ch.to_string().as_str() {
			"=" => {
				Token { token_type: TokenKind::ASSIGN(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			";" => {
				Token { token_type: TokenKind::SEMICOLON(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			"(" => {
				Token { token_type: TokenKind::LPAREN(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			")" => {
				Token { token_type: TokenKind::RPAREN(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			"," => {
				Token { token_type: TokenKind::COMMA(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			"+" => {
				Token { token_type: TokenKind::PLUS(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			"{" => {
				Token { token_type: TokenKind::LBRACE(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			"}" => {
				Token { token_type: TokenKind::RBRACE(self.ch.to_string()),  literal: self.ch.to_string()  }
			}
			_ => {
				Token { token_type: TokenKind::EOF("".to_string()),  literal: "".to_string()  }
			}
		};

		self.read_char();

		token
	}
}