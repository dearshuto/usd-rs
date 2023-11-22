use std::{iter::Peekable, slice::Iter};

use super::Token;

// def Mesh "name" {
//   key = value
// }
pub struct Property {
    pub tokens: Vec<Token>,
}

pub struct Definition {
    pub identifier: Option<String>,
    pub name: String,
    pub properties: Vec<Property>,
}

pub enum Node {
    Version(u32),
    Definition(Definition),
}

pub struct SyntaxTree {
    pub nodes: Vec<Node>,
}

impl SyntaxTree {
    pub fn new(tokens: &[Token]) -> Self {
        let mut nodes = Vec::default();
        let mut token_stream = tokens.iter().peekable();
        while token_stream.peek().is_some() {
            let token = token_stream.peek().unwrap();
            match token {
                Token::Directive(_) => {
                    let node = Self::parse_directive(&mut token_stream);
                    nodes.push(node);
                }
                Token::Def => {
                    let node = Self::parse_definition(&mut token_stream);
                    nodes.push(node);
                }
                _ => {
                    token_stream.next();
                }
            }
        }

        Self { nodes }
    }

    fn parse_directive(token_stream: &mut Peekable<Iter<Token>>) -> Node {
        let Some(token) = token_stream.next() else {
            panic!()
        };
        let Token::Directive(d) = token else {
            panic!();
        };
        if d != "usda" {
            panic!()
        }

        let Token::NumberValue(version) = token_stream.next().unwrap() else {
            panic!()
        };

        Node::Version(*version as u32)
    }

    fn parse_definition(token_stream: &mut Peekable<Iter<Token>>) -> Node {
        let Some(def_token) = token_stream.next() else {
            panic!();
        };

        let Token::Def = def_token.clone() else {
            panic!()
        };

        let (identifier, name) = match token_stream.next().unwrap() {
            Token::Identifier(identifier) => {
                let Token::StringValue(name) = token_stream.next().unwrap() else {
                    panic!();
                };
                (Some(identifier.clone()), name.clone())
            }
            Token::StringValue(name) => (None, name.clone()),
            _ => panic!(),
        };

        // 中かっこまで読み飛ばす
        // 本来は () で囲まれたプロパティがある
        while token_stream.next().unwrap() == &Token::BraceL {
            continue;
        }

        let properties = Self::consume_properties(token_stream);

        let definition = Definition {
            identifier,
            name,
            properties,
        };
        Node::Definition(definition)
    }

    fn consume_properties(token_stream: &mut Peekable<Iter<Token>>) -> Vec<Property> {
        let Some(_begin) = token_stream.peek() else {
            return Vec::new();
        };
        // let Token::BraceL = begin else { panic!() };

        let mut result = Vec::default();

        loop {
            let mut tokens = Vec::new();
            loop {
                let Some(token) = token_stream.next() else {
                    panic!();
                };

                if &Token::Return == token {
                    // 行の終わりまでがプロパティ
                    break;
                };

                tokens.push(token.clone());
            }
            result.push(Property { tokens });

            // 中かっこの終わりまできたら終了
            let Some(next_token) = token_stream.peek() else {
                break;
            };
            if next_token == &&Token::BraceR {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::{analyzer::Lexer, Token};

    use super::{Node, SyntaxTree};

    #[test]
    fn def() {
        let input = r#"def Xform "string_value" {}"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let Node::Definition(definition) = &syntax_tree.nodes[0] else {
            panic!()
        };

        if let Some(identifier) = &definition.identifier {
            assert_eq!(identifier, "Xform");
        } else {
            panic!();
        }

        assert_eq!(definition.name, "string_value");
    }

    #[test]
    fn properties() {
        let input = r#"def Xform "string_value" {
                        color3f[] primvars:displayColor = [(0, 0, 1)]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);

        // def 部分のテストは def() で検証しているのでスキップ
        let Node::Definition(definition) = &syntax_tree.nodes[0] else {
            panic!()
        };
        assert_eq!(definition.properties.len(), 1);

        let property = &definition.properties[0];
        assert_eq!(property.tokens[0], Token::Color3fArray);
        assert_eq!(
            property.tokens[1],
            Token::Identifier("primvars:displayColor".to_string())
        );
        assert_eq!(property.tokens[2], Token::Equal);
        assert_eq!(property.tokens[3], Token::BracketL);
        assert_eq!(property.tokens[4], Token::ParenthesesL);
        assert_eq!(property.tokens[5], Token::NumberValue(0.0));
        assert_eq!(property.tokens[6], Token::Comma);
        assert_eq!(property.tokens[7], Token::NumberValue(0.0));
        assert_eq!(property.tokens[8], Token::Comma);
        assert_eq!(property.tokens[9], Token::NumberValue(1.0));
        assert_eq!(property.tokens[10], Token::ParenthesesR);
        assert_eq!(property.tokens[11], Token::BracketR);
    }
}
