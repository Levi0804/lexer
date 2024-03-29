use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,         // current position in input (points to current char)
    read_position: usize,    // current reading position in input (after current char)
    ch: u8,                  // current char in examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self { input, position: 0, read_position: 0, ch: 0 };
        l.read_char();
        return l;
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position]; // only ascii
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

	pub fn next_token(&mut self) -> Token  {
		let literal = char::from_u32(self.ch as u32).unwrap().to_string();

		let token: Token = match &literal[..] {
			"=" => {
				Token { token_type: TokenKind::ASSIGN ,  literal  }
			}
			";" => {
				Token { token_type: TokenKind::SEMICOLON,  literal  }
			}
			"(" => {
				Token { token_type: TokenKind::LPAREN,  literal  }
			}
			")" => {
				Token { token_type: TokenKind::RPAREN,  literal  }
			}
			"," => {
				Token { token_type: TokenKind::COMMA,  literal  }
			}
			"+" => {
				Token { token_type: TokenKind::PLUS,  literal  }
			}
			"{" => {
				Token { token_type: TokenKind::LBRACE,  literal  }
			}
			"}" => {
				Token { token_type: TokenKind::RBRACE,  literal  }
			}
			_ => {
				Token { token_type: TokenKind::EOF,  literal: "".to_string()  }
			}
		};

		self.read_char();

		token
	}
}