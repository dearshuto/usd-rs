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
        assert_eq!(lexer.next(), Some(Ok(Token::ParenthesesL)));
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(Token::Comma)));
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(Token::Comma)));
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(Token::ParenthesesR)));
        assert_eq!(lexer.next(), Some(Ok(Token::BraceR)));
    }

    // #[test]
    // fn string() {
    //     let input = r#""string_value""#;
    //     let token = &Lexer::tokenize(input)[0];
    //     let expected = Token::StringValue("string_value".to_string());
    //     assert_eq!(expected, *token);
    // }

    // #[test]
    // fn number() {
    //     let input = r#"12345, 12, 345"#;
    //     let tokens = &Lexer::tokenize(input);
    //     assert_eq!(Token::NumberValue(12345.0), tokens[0]);
    //     assert_eq!(Token::Comma, tokens[1]);
    //     assert_eq!(Token::NumberValue(12.0), tokens[2]);
    //     assert_eq!(Token::Comma, tokens[3]);
    //     assert_eq!(Token::NumberValue(345.0), tokens[4]);
    // }

    // #[test]
    // fn keywords() {
    //     let input = r#"color3f float3[] uniform int[]"#;
    //     let tokens = &Lexer::tokenize(input);
    //     assert_eq!(Token::Color3f, tokens[0]);
    //     assert_eq!(Token::Float3fArray, tokens[1]);
    //     assert_eq!(Token::Uniform, tokens[2]);
    //     assert_eq!(Token::IntArray, tokens[3]);
    // }

    // #[test]
    // fn string_and_numeric() {
    //     let input = r#"12345 "my_string""#;
    //     let mut tokens = Lexer::tokenize(input).into_iter();
    //     let tests = vec![
    //         Token::NumberValue(12345.0),                 // 12345
    //         Token::StringValue("my_string".to_string()), // "my_string"
    //     ];

    //     for test in tests {
    //         let value = tokens.next().unwrap();
    //         assert_eq!(value, test);
    //     }
    // }

    // #[test]
    // fn simple_directive() {
    //     let input = r#"#usda"#;
    //     let token = &Lexer::tokenize(input)[0];
    //     let expected = Token::Directive("usda".to_string());
    //     assert_eq!(*token, expected);
    // }

    // #[test]
    // fn simple_def() {
    //     let input = r#"def Xform "root" {}"#;
    //     let tokens = Lexer::tokenize(input);
    //     let tests = vec![
    //         Token::Def,
    //         Token::Identifier("Xform".to_string()),
    //         Token::StringValue("root".to_string()),
    //         Token::BraceL, // {
    //         Token::BraceR, // }
    //     ];

    //     for index in 0..tests.len() {
    //         let expected = &tests[index];
    //         let value = &tokens[index];
    //         assert_eq!(value, expected);
    //     }
    // }

    // #[test]
    // fn simple_def_multiline() {
    //     let input = r#"def Xform "root"
    //         {

    //         }"#;
    //     let tokens = Lexer::tokenize(input);
    //     let tests = vec![
    //         Token::Def,
    //         Token::Identifier("Xform".to_string()),
    //         Token::StringValue("root".to_string()),
    //         Token::Return,
    //         Token::BraceL, // {
    //         Token::Return,
    //         Token::Return,
    //         Token::BraceR, // }
    //     ];

    //     for index in 0..tests.len() {
    //         let expected = &tests[index];
    //         let value = &tokens[index];
    //         assert_eq!(value, expected);
    //     }
    // }
}
