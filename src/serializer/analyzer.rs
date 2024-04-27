use logos::Logos;

use super::tokens::Token;

pub trait ILexer {
    fn next(&mut self) -> Option<Result<Token, ()>>;

    fn peek(&self) -> &str;
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            lexer: Token::lexer(str),
        }
    }
}

impl<'a> ILexer for Lexer<'a> {
    fn next(&mut self) -> Option<Result<Token, ()>> {
        self.lexer.next()
    }

    fn peek(&self) -> &str {
        self.lexer.slice()
    }
}

#[cfg(test)]
mod tests {
    use super::{ILexer, Lexer, Token};

    #[test]
    fn simple() {
        let input = r#"def Xform "root"
            {
                double3 xformOp:translate = (1, 2, -0.75)
            }"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Def)));
        assert_eq!(lexer.next(), Some(Ok(Token::Xform)));

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.peek(), "\"root\"");

        assert_eq!(lexer.next(), Some(Ok(Token::BraceL)));
        assert_eq!(lexer.next(), Some(Ok(Token::Double3f)));
        assert_eq!(lexer.next(), Some(Ok(Token::XformOp)));
        assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
        assert_eq!(lexer.next(), Some(Ok(Token::Translate)));
        assert_eq!(lexer.next(), Some(Ok(Token::Equal)));
        assert_eq!(lexer.next(), Some(Ok(Token::NumberArray)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceR)));

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn string() {
        let input = r#""string_value""#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.peek(), "\"string_value\"");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn number_natural() {
        let input = r#"12345"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.peek(), "12345");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn number_real_positive() {
        let input = r#"54.2"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.peek(), "54.2");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn number_real_negative() {
        let input = r#"-0.45"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.peek(), "-0.45");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn number_negative() {
        let input = r#"-345"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.peek(), "-345");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn number_array() {
        let input = r#"(12345.0, -1.2, 3.45)"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Ok(Token::NumberArray)));
        assert_eq!(lexer.peek(), "(12345.0, -1.2, 3.45)");

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn keywords() {
        let input = r#"color3f float3[] uniform int[]"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Color3f)));
        assert_eq!(lexer.next(), Some(Ok(Token::Float3fArray)));
        assert_eq!(lexer.next(), Some(Ok(Token::Uniform)));
        assert_eq!(lexer.next(), Some(Ok(Token::IntArray)));

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn string_and_numeric() {
        let input = r#"12345 "my_string""#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.peek(), "12345");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn simple_directive() {
        let input = r#"#usda"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Usda)));

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn simple_def() {
        let input = r#"def Xform "root" {}"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Def)));
        assert_eq!(lexer.next(), Some(Ok(Token::Xform)));
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceL)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceR)));

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn simple_def_multiline() {
        let input = r#"def Xform "root"
            {

            }"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Def)));
        assert_eq!(lexer.next(), Some(Ok(Token::Xform)));
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceL)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceR)));

        assert_eq!(lexer.next(), None);
    }
}
