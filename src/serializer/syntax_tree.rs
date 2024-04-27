use std::{iter::Peekable, slice::Iter};

use eframe::glow::NativeFramebuffer;

use super::{analyzer::ILexer, Token};

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
    pub fn new_with<TLexer: ILexer>(mut lexer: TLexer) -> Self {
        let mut nodes = Vec::default();

        while let Some(result) = lexer.next() {
            let Ok(token) = result else {
                continue;
            };

            match token {
                Token::Def => {
                    let node = Self::parse_definition(token, &mut lexer);
                    nodes.push(node);
                }
                Token::XformOp => {}
                _ => {}
            }
        }

        Self { nodes }
    }

    pub fn new(tokens: &[Token]) -> Self {
        let mut nodes = Vec::default();
        // let mut token_stream = tokens.iter().peekable();
        // while token_stream.peek().is_some() {
        //     let token = token_stream.peek().unwrap();
        //     match token {
        //         Token::Directive(_) => {
        //             let node = Self::parse_directive(&mut token_stream);
        //             nodes.push(node);
        //         }
        //         Token::Def => {
        //             let node = Self::parse_definition(&mut token_stream);
        //             nodes.push(node);
        //         }
        //         _ => {
        //             token_stream.next();
        //         }
        //     }
        // }

        Self { nodes }
    }

    fn parse_directive<TLexer: ILexer>(current_token: Token, mut lexer: TLexer) -> Node {
        todo!()
        //     let Some(token) = token_stream.next() else {
        //         panic!()
        //     };

        //     token.
        //     let Token::Text(d) = token else {
        //         panic!();
        //     };
        //     if d != "usda" {
        //         panic!()
        //     }

        //     let Token::NumberValue(version) = token_stream.next().unwrap() else {
        //         panic!()
        //     };

        //     Node::Version(*version as u32)
    }

    fn parse_definition<TLexer: ILexer>(current_token: Token, lexer: &mut TLexer) -> Node {
        // "def xxx xxx {}" のブロックをパースするので def 始まりであることを確認
        if current_token != Token::Def {
            panic!()
        }

        // "def Xform xxx xxx{}"
        let Some(Ok(Token::Xform)) = lexer.next() else {
            panic!();
        };
        let identifier = Some(lexer.peek().to_string());

        // "def Xform "name" xxx{}"
        let Some(Ok(Token::Text)) = lexer.next() else {
            panic!();
        };

        // 文字はダブルクォーテーションを含むのでそれを取り除く
        let name = lexer.peek();
        let name = name[1..(name.len() - 1)].to_string();

        // 本来は () で囲まれたプロパティがあるが、
        // プロパティのブロックが始まることを期待して中かっこであることを要求
        let Some(Ok(bracel_token)) = lexer.next() else {
            panic!();
        };
        if bracel_token != Token::BraceL {
            panic!();
        }

        let properties = Self::consume_properties(bracel_token, lexer);

        let definition = Definition {
            identifier,
            name,
            properties,
        };
        Node::Definition(definition)
    }

    fn consume_properties<TLexer: ILexer>(
        current_token: Token,
        lexer: &mut TLexer,
    ) -> Vec<Property> {
        let Token::BraceL = current_token else {
            panic!()
        };

        let mut result = Vec::default();

        // color3f[] xxx xx xx
        let Some(Ok(Token::Color3fArray)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars xxx xx xx
        let Some(Ok(Token::IdentifierTentative)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars: xxx xx xx
        let Some(Ok(Token::Colon)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor xx xx
        let Some(Ok(Token::IdentifierTentative)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor =
        let Some(Ok(Token::Equal)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor =
        let Some(Ok(Token::Equal)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor = [
        let Some(Ok(Token::ParenthesesL)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor = [(0, 0, 1)
        let Some(Ok(Token::NumberArray)) = lexer.next() else {
            panic!();
        };

        // color3f[] primvars:displayColor = [(0, 0, 1)]
        let Some(Ok(Token::ParenthesesR)) = lexer.next() else {
            panic!();
        };

        // } で終わってることを確認
        let Some(Ok(Token::BracketR)) = lexer.next() else {
            panic!();
        };

        result.push(Property {
            tokens: vec![Token::Color3fArray],
        });

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
        let lexer = Lexer::new(input);
        let syntax_tree = SyntaxTree::new_with(lexer);
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
        let tokens = Lexer::new(input);
        let syntax_tree = SyntaxTree::new_with(tokens);

        // def 部分のテストは def() で検証しているのでスキップ
        let Node::Definition(definition) = &syntax_tree.nodes[0] else {
            panic!()
        };
        assert_eq!(definition.properties.len(), 1);

        // let property = &definition.properties[0];
        // assert_eq!(property.tokens[0], Token::Color3fArray);
        // assert_eq!(
        //     property.tokens[1],
        //     Token::Identifier("primvars:displayColor".to_string())
        // );
        // assert_eq!(property.tokens[2], Token::Equal);
        // assert_eq!(property.tokens[3], Token::BracketL);
        // assert_eq!(property.tokens[4], Token::ParenthesesL);
        // // assert_eq!(property.tokens[5], Token::NumberValue(0.0));
        // assert_eq!(property.tokens[6], Token::Comma);
        // // assert_eq!(property.tokens[7], Token::NumberValue(0.0));
        // assert_eq!(property.tokens[8], Token::Comma);
        // // assert_eq!(property.tokens[9], Token::NumberValue(1.0));
        // assert_eq!(property.tokens[10], Token::ParenthesesR);
        // assert_eq!(property.tokens[11], Token::BracketR);
    }
}
