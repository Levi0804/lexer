use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: String,
    // current position in input (points to current char)
    position: usize,      
    // current reading position in input (after current char)
    read_position: usize, 
    // current char in examination
    ch: u8,               
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let literal = char::from_u32(self.ch as u32).unwrap().to_string();

        let mut token = Token {
            literal: "".to_string(),
            token_type: TokenKind::EOF,
        };

        token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    Token {
                        token_type: TokenKind::EQ,
                        literal: char::from_u32((ch) as u32).unwrap().to_string()
                            + &char::from_u32((self.ch) as u32).unwrap().to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenKind::ASSIGN,
                        literal,
                    }
                }
            }
            b';' => Token {
                token_type: TokenKind::SEMICOLON,
                literal,
            },
            b'(' => Token {
                token_type: TokenKind::LPAREN,
                literal,
            },
            b')' => Token {
                token_type: TokenKind::RPAREN,
                literal,
            },
            b',' => Token {
                token_type: TokenKind::COMMA,
                literal,
            },
            b'+' => Token {
                token_type: TokenKind::PLUS,
                literal,
            },
            b'{' => Token {
                token_type: TokenKind::LBRACE,
                literal,
            },
            b'}' => Token {
                token_type: TokenKind::RBRACE,
                literal,
            },
            b'-' => Token {
                token_type: TokenKind::MINUS,
                literal,
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    Token {
                        token_type: TokenKind::NOTEQ,
                        literal: char::from_u32((ch) as u32).unwrap().to_string()
                            + &char::from_u32((self.ch) as u32).unwrap().to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenKind::BANG,
                        literal,
                    }
                }
            }
            b'*' => Token {
                token_type: TokenKind::ASTERISK,
                literal,
            },
            b'/' => Token {
                token_type: TokenKind::SLASH,
                literal,
            },
            b'<' => Token {
                token_type: TokenKind::LT,
                literal,
            },
            b'>' => Token {
                token_type: TokenKind::GT,
                literal,
            },
            b'\0' => Token {
                token_type: TokenKind::EOF,
                literal: "".to_string(),
            },
            _ => {
                if self.is_letter() {
                    token.literal = self.read_identifier();
                    token.token_type = self.look_up_identifier(token.literal.clone());
                    token
                } else if self.is_digit() {
                    Token {
                        literal: self.read_number(),
                        token_type: TokenKind::INT,
                    }
                } else {
                    Token {
                        token_type: TokenKind::ILLEGAL,
                        literal,
                    }
                }
            }
        };

        self.read_char();

        token
    }

    fn is_letter(&self) -> bool {
        self.ch >= 97 && self.ch <= 122 || self.ch >= 65 && self.ch <= 90 || self.ch == 95
    }

    fn is_digit(&self) -> bool {
        self.ch >= 48 && self.ch <= 57
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn look_up_identifier(&self, ident: String) -> TokenKind {
        if ident == "fn" {
            TokenKind::FUNCTION
        } else if ident == "let" {
            TokenKind::LET
        } else if ident == "true" {
            TokenKind::TRUE
        } else if ident == "false" {
            TokenKind::FALSE
        } else if ident == "if" {
            TokenKind::IF
        } else if ident == "else" {
            TokenKind::ELSE
        } else if ident == "return" {
            TokenKind::RETURN
        } else {
            TokenKind::IDENT
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b'\t' || self.ch == b' ' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position > self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
}
