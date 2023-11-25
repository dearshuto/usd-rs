use std::iter::Peekable;
use std::str::Chars;

use super::tokens::{Token, CHAR_TOKENS, KEYWORD_TOKENS};

pub struct Lexer;

impl Lexer {
    pub fn tokenize(string: &str) -> Vec<Token> {
        let mut tokens = Vec::default();

        let mut char_stream = string.chars().peekable();

        while char_stream.peek().is_some() {
            let cc = char_stream.peek().unwrap();
            match cc {
                ' ' => {
                    // 空白は区切り文字的な扱いなのでトークン化はしない
                    let _token = Self::consume_white_space(&mut char_stream);
                    // tokens.push(token);
                }
                '#' => {
                    let token = Self::consume_directive(&mut char_stream);
                    tokens.push(token);
                }
                '"' => {
                    let token = Self::consume_string(&mut char_stream);
                    tokens.push(token);
                }
                '-' => {
                    // ハイフンはマイナスの数値の目印とする
                    let token = Self::consume_number(&mut char_stream);
                    tokens.push(token);
                }
                '0'..='9' => {
                    let token = Self::consume_number(&mut char_stream);
                    tokens.push(token);
                }
                'a'..='z' | 'A'..='Z' => {
                    let token = Self::consume_keyword(&mut char_stream);
                    tokens.push(token);
                }
                '\n' | '\r' => {
                    let _ = char_stream.next();
                    tokens.push(Token::Return);
                }
                _ => {
                    let Some(token) = CHAR_TOKENS.get(cc) else {
                        char_stream.next();
                        continue;
                    };

                    tokens.push(token.clone());
                    char_stream.next();
                }
            }
        }

        tokens
    }

    fn consume_keyword(iter: &mut Peekable<Chars>) -> Token {
        let Some(begin) = iter.peek() else { panic!() };
        if !begin.is_alphabetic() {
            panic!()
        }

        // 空白までの単語を抽出
        let mut result = String::new();
        loop {
            let Some(c) = iter.peek() else {
                break;
            };

            if c == &' ' {
                break;
            }

            let c = iter.next().unwrap();
            result.push(c);
        }

        if let Some(token) = KEYWORD_TOKENS.get(&result) {
            token.clone()
        } else {
            Token::Identifier(result)
        }
    }

    fn consume_string(iter: &mut Peekable<Chars>) -> Token {
        let Some(begin) = iter.next() else { panic!() };
        if begin != '"' {
            panic!()
        }

        let mut result = String::new();
        loop {
            match iter.next() {
                Some('"') => break,
                Some(c) => result.push(c),
                None => panic!(),
            }
        }
        Token::StringValue(result)
    }

    fn consume_number(iter: &mut Peekable<Chars>) -> Token {
        let mut result = String::new();

        // マイナスの値だとハイフン始まりなので最初に消費しておく
        if let Some(c) = iter.peek() {
            if c == &'-' {
                let c = iter.next().unwrap();
                result.push(c);
            }
        }

        while let Some(c) = iter.peek() {
            if c.is_numeric() || c == &'.' || c == &'-' || c == &'e' {
                let c = iter.next().unwrap();
                result.push(c);
                continue;
            } else {
                break;
            }
        }

        let numeric = result.parse::<f64>().unwrap();
        Token::NumberValue(numeric)
    }

    fn consume_white_space(iter: &mut Peekable<Chars>) -> Token {
        let Some(begin) = iter.peek() else { panic!() };
        if *begin != ' ' {
            panic!()
        }

        loop {
            match iter.peek() {
                Some(' ') => {
                    iter.next();
                    continue;
                }
                Some(_) => break,
                None => break,
            }
        }
        Token::Space
    }

    fn consume_directive(iter: &mut Peekable<Chars>) -> Token {
        let Some(begin) = iter.peek() else { panic!() };
        if *begin != '#' {
            panic!()
        }

        let mut result = String::new();
        loop {
            match iter.peek() {
                Some('#') => {
                    iter.next();
                    continue;
                }
                Some(' ') => break,
                Some(_) => {
                    let c = iter.next().unwrap();
                    result.push(c);
                    continue;
                }
                None => break,
            }
        }
        Token::Directive(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn simple() {
        let input = r#"def Xform "root"
            {
                double3 xformOp:translate = (1, 2, -0.75)
            }"#;
        let tokens = Lexer::tokenize(input);
        let tests = vec![
            Token::Def,
            Token::Identifier("Xform".to_string()),
            Token::StringValue("root".to_string()),
            Token::Return,
            Token::BraceL, // {
            Token::Return,
            Token::Double3f,
            Token::Identifier("xformOp:translate".to_string()),
            Token::Equal,
            Token::ParenthesesL,
            Token::NumberValue(1.0),
            Token::Comma,
            Token::NumberValue(2.0),
            Token::Comma,
            Token::NumberValue(-0.75),
            Token::ParenthesesR,
            Token::Return,
            Token::BraceR, // }
        ];

        for index in 0..tests.len() {
            let expected = &tests[index];
            let value = &tokens[index];
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn string() {
        let input = r#""string_value""#;
        let token = &Lexer::tokenize(input)[0];
        let expected = Token::StringValue("string_value".to_string());
        assert_eq!(expected, *token);
    }

    #[test]
    fn number() {
        let input = r#"12345, 12, 345, 3.061617e-17"#;
        let tokens = &Lexer::tokenize(input);
        assert_eq!(Token::NumberValue(12345.0), tokens[0]);
        assert_eq!(Token::Comma, tokens[1]);
        assert_eq!(Token::NumberValue(12.0), tokens[2]);
        assert_eq!(Token::Comma, tokens[3]);
        assert_eq!(Token::NumberValue(345.0), tokens[4]);
        assert_eq!(Token::Comma, tokens[5]);
        assert_eq!(Token::NumberValue(3.061617e-17), tokens[6]);
    }

    #[test]
    fn keywords() {
        let input = r#"color3f float3[] uniform int[] normal3f[]"#;
        let tokens = &Lexer::tokenize(input);
        assert_eq!(Token::Color3f, tokens[0]);
        assert_eq!(Token::Float3fArray, tokens[1]);
        assert_eq!(Token::Uniform, tokens[2]);
        assert_eq!(Token::IntArray, tokens[3]);
        assert_eq!(Token::Normal3fArray, tokens[4]);
    }

    #[test]
    fn string_and_numeric() {
        let input = r#"12345 "my_string""#;
        let mut tokens = Lexer::tokenize(input).into_iter();
        let tests = vec![
            Token::NumberValue(12345.0),                 // 12345
            Token::StringValue("my_string".to_string()), // "my_string"
        ];

        for test in tests {
            let value = tokens.next().unwrap();
            assert_eq!(value, test);
        }
    }

    #[test]
    fn simple_directive() {
        let input = r#"#usda"#;
        let token = &Lexer::tokenize(input)[0];
        let expected = Token::Directive("usda".to_string());
        assert_eq!(*token, expected);
    }

    #[test]
    fn simple_def() {
        let input = r#"def Xform "root" {}"#;
        let tokens = Lexer::tokenize(input);
        let tests = vec![
            Token::Def,
            Token::Identifier("Xform".to_string()),
            Token::StringValue("root".to_string()),
            Token::BraceL, // {
            Token::BraceR, // }
        ];

        for index in 0..tests.len() {
            let expected = &tests[index];
            let value = &tokens[index];
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn simple_def_multiline() {
        let input = r#"def Xform "root"
            {

            }"#;
        let tokens = Lexer::tokenize(input);
        let tests = vec![
            Token::Def,
            Token::Identifier("Xform".to_string()),
            Token::StringValue("root".to_string()),
            Token::Return,
            Token::BraceL, // {
            Token::Return,
            Token::Return,
            Token::BraceR, // }
        ];

        for index in 0..tests.len() {
            let expected = &tests[index];
            let value = &tokens[index];
            assert_eq!(value, expected);
        }
    }
}
