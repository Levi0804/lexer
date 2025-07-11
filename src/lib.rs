pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_lexer() {
        let input = "let add = fn(x, y) { x + y; };";

        let mut lexer = Lexer::new(input.to_string());
        let mut v: Vec<Token> = Vec::new();
        let mut token = lexer.next_token();

        while token.token_type != TokenKind::EOF {
            v.push(token);
            token = lexer.next_token();
        }

        assert_eq!(
            v,
            vec![
                Token {
                    token_type: TokenKind::LET,
                    literal: String::from("let"),
                },
                Token {
                    token_type: TokenKind::IDENT,
                    literal: String::from("add"),
                },
                Token {
                    token_type: TokenKind::ASSIGN,
                    literal: String::from("="),
                },
                Token {
                    token_type: TokenKind::FUNCTION,
                    literal: String::from("fn"),
                },
                Token {
                    token_type: TokenKind::IDENT,
                    literal: String::from("x"),
                },
                Token {
                    token_type: TokenKind::IDENT,
                    literal: String::from("y"),
                },
                Token {
                    token_type: TokenKind::LBRACE,
                    literal: String::from("{"),
                },
                Token {
                    token_type: TokenKind::IDENT,
                    literal: String::from("x"),
                },
                Token {
                    token_type: TokenKind::PLUS,
                    literal: String::from("+"),
                },
                Token {
                    token_type: TokenKind::IDENT,
                    literal: String::from("y"),
                },
                Token {
                    token_type: TokenKind::RBRACE,
                    literal: String::from("}"),
                },
                Token {
                    token_type: TokenKind::SEMICOLON,
                    literal: String::from(";"),
                },
            ]
        );
    }
}
