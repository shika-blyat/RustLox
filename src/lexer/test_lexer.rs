#[cfg(test)]
mod tests {
    use crate::lexer::lexer::{Scanner, Token};
    use crate::lexer::tokens::TokenType;
    #[test]
    fn unit_token() {
        let expected_result = vec![
            Token::new(TokenType::LeftParen, String::from("("), 1),
            Token::new(TokenType::RightParen, String::from(")"), 1),
            Token::new(TokenType::LeftBrace, String::from("{"), 1),
            Token::new(TokenType::RightBrace, String::from("}"), 1),
            Token::new(TokenType::Comma, String::from(","), 1),
            Token::new(TokenType::Dot, String::from("."), 1),
            Token::new(TokenType::Plus, String::from("+"), 1),
            Token::new(TokenType::Minus, String::from("-"), 1),
            Token::new(TokenType::Semicolon, String::from(";"), 1),
            Token::new(TokenType::Eof, String::new(), 1),
        ];
        let mut scanner = Scanner::new("(){},.+-;");
        assert_eq!(scanner.scan_tokens(), expected_result);
    }
    #[test]
    fn multi_char_token(){
        let expected_result = vec![
            Token::new(TokenType::BangEqual, String::from("!="), 1),
            Token::new(TokenType::Number(2.0), String::from("2"), 1),
            Token::new(TokenType::Slash, String::from("/"), 1),
            Token::new(TokenType::Number(4.0), String::from("4"), 1),
            Token::new(TokenType::Star, String::from("*"), 1),
            Token::new(TokenType::Identifier("abc".to_string()), String::from("abc"), 1),
            Token::new(TokenType::Eof, String::new(), 1),
        ];
        let mut scanner = Scanner::new("!= 2/4*abc//(){");
        assert_eq!(scanner.scan_tokens(), expected_result);
    }
    #[test]
    fn multi_line_token(){
        let expected_result = vec![
            Token::new(TokenType::String("a\nbc".to_string()), String::from("a\nbc"), 2),
            Token::new(TokenType::String("abc".to_string()), String::from("abc"), 3),
            Token::new(TokenType::Eof, String::new(), 3),
        ];
        let mut scanner = Scanner::new("\"a\nbc\"\n/*abc \n */\"abc\"");
        assert_eq!(scanner.scan_tokens(), expected_result);
    }
}