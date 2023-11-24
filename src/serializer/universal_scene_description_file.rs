use std::{iter::Peekable, slice::Iter};

use super::{
    syntax_tree::{Node, SyntaxTree},
    types::{PropertyType, UpAxis},
    Token,
};

#[derive(Debug)]
pub struct Header {
    pub default_prim: String,
    pub document: String,
    pub meters_per_unit: f32,
    pub up_axis: UpAxis,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            default_prim: String::new(),
            document: String::new(),
            meters_per_unit: 1.0,
            up_axis: UpAxis::Z,
        }
    }
}

pub struct Property {
    pub identifier: String,
    pub property: PropertyType,
}

pub struct Definition {
    pub name: String,
    pub properties: Vec<Property>,
}

pub struct UniversalSceneDescriptionFile {
    header: Header,
    definitions: Vec<Definition>,
}

impl UniversalSceneDescriptionFile {
    pub fn new(syntax_tree: &SyntaxTree) -> Self {
        let mut node_stream = syntax_tree.nodes.iter();
        let _version = node_stream.next();

        let definitions = node_stream
            .map(|node| {
                let Node::Definition(definitions) = node else {
                    panic!()
                };
                Self::parse_definitions(definitions)
            })
            .collect::<Vec<Definition>>();

        Self {
            header: Header::default(),
            definitions,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    fn parse_definitions(definition: &super::syntax_tree::Definition) -> Definition {
        let properties = definition
            .properties
            .iter()
            .filter_map(Self::parse_property)
            .collect::<Vec<Property>>();
        Definition {
            name: definition.name.to_string(),
            properties,
        }
    }

    fn parse_property(property: &super::syntax_tree::Property) -> Option<Property> {
        let mut token_stream = property.tokens.iter().peekable();
        if token_stream.peek().is_none() {
            panic!()
        };

        // 先頭のタイプを消費
        match token_stream.next().unwrap() {
            Token::Color3fArray => None,
            Token::Float3f => todo!(),
            Token::Float3fArray => todo!(),
            Token::Double3f => {
                let Token::Identifier(identifier) = token_stream.next().unwrap() else {
                    panic!()
                };
                let _equal = token_stream.next();
                let property = PropertyType::XformOpTranslate([0.0, 0.0, 0.0]);
                Some(Property {
                    identifier: identifier.to_string(),
                    property,
                })
            }
            Token::IntArray => {
                let Token::Identifier(identifier) = token_stream.next().unwrap() else {
                    panic!()
                };
                let _equal = token_stream.next();
                let data = Self::comsume_number_array(&mut token_stream);
                let property = match &identifier[..] {
                    "faceVertexCounts" => PropertyType::FaceVertexCounts(data),
                    "faceVertexIndices" => PropertyType::FaceVertexIndicies(data),
                    _ => panic!(),
                };
                Some(Property {
                    identifier: identifier.clone(),
                    property,
                })
            }
            Token::Point3f => todo!(),
            Token::Point3fArray => {
                let Token::Identifier(identifier) = token_stream.next().unwrap() else {
                    panic!()
                };
                let _equal = token_stream.next();
                let data = Self::comsume_packed_number_array(&mut token_stream);
                let property = PropertyType::Points(data);
                Some(Property {
                    identifier: identifier.clone(),
                    property,
                })
            }
            Token::Normal3fArray => {
                let Token::Identifier(identifier) = token_stream.next().unwrap() else {
                    panic!()
                };
                let _equal = token_stream.next();
                let data = Self::comsume_packed_number_array(&mut token_stream);
                let property = PropertyType::Normals(data);
                Some(Property {
                    identifier: identifier.clone(),
                    property,
                })
            }
            Token::Uniform => todo!(),
            _ => None,
        }
    }

    // [0, 1, 2, 3] を消費
    fn comsume_number_array<T: num_traits::NumCast>(
        token_stream: &mut Peekable<Iter<Token>>,
    ) -> Vec<T> {
        Self::comsume_number_array_impl(token_stream, Token::BracketL, Token::BracketR)
    }

    // [(0, 1, 2, 3), (3, 4, 5)]
    fn comsume_packed_number_array<T: num_traits::NumCast + Copy, const COUNT: usize>(
        token_stream: &mut Peekable<Iter<Token>>,
    ) -> Vec<[T; COUNT]> {
        // "[" で始まってなかったらパニック
        let Some(begin) = token_stream.peek() else {
            panic!();
        };
        if begin != &&Token::BracketL {
            panic!();
        }
        let _ = token_stream.next();

        let mut numbers = Vec::default();
        loop {
            let Some(current) = token_stream.peek() else {
                panic!();
            };

            match current {
                Token::BracketR => {
                    // "]" まで到達した
                    let _ = token_stream.next();
                    break;
                }
                Token::ParenthesesL => {
                    // "(" が見つかったので数字の配列として消費
                    // (0, 1, 2, 3) のように ")" までが消費される
                    let array = Self::comsume_number_array_impl::<T>(
                        token_stream,
                        Token::ParenthesesL,
                        Token::ParenthesesR,
                    );
                    let mut temp = [T::from(0).unwrap(); COUNT];
                    (0..COUNT).for_each(|index| {
                        temp[index] = array[index];
                    });
                    numbers.push(temp);
                    continue;
                }
                Token::Comma => {
                    // 区切り文字だった
                    let _ = token_stream.next();
                    continue;
                }
                _ => {
                    // 謎のトークンがきたらエラー
                    panic!()
                }
            };
        }

        numbers
    }

    // 指定の囲い文字の範囲にある数字を消費します。
    // [0, 1, 2, 3] -> Self::comsume_number_array_impl(token_stream, Token::BracketL, Token::BraceR)
    // (0, 1, 2, 3) -> Self::comsume_number_array_impl(token_stream, Token::ParenthesesL, Token::ParenthesesR)
    fn comsume_number_array_impl<T: num_traits::NumCast>(
        token_stream: &mut Peekable<Iter<Token>>,
        range_token_begin: Token,
        range_token_end: Token,
    ) -> Vec<T> {
        // 指定のトークンで始まってなかったらパニック
        let Some(begin) = token_stream.peek() else {
            panic!();
        };
        if begin != &&range_token_begin {
            panic!();
        }
        // 始まりのトークンを消費
        let _ = token_stream.next();

        let mut numbers = Vec::default();
        loop {
            let Some(current) = token_stream.next() else {
                panic!();
            };

            match current {
                Token::BracketL => {
                    // 指定のトークンまで到達した
                    break;
                }
                Token::Comma => {
                    // 区切り文字だった
                    continue;
                }
                Token::NumberValue(number) => {
                    let casted = T::from(*number).unwrap();
                    numbers.push(casted);
                    continue;
                }
                _ => {
                    if range_token_end == *current {
                        // 指定のトークンまで到達した
                        break;
                    }

                    // 謎のトークンだったらエラー
                    panic!()
                }
            }
        }

        numbers
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::{analyzer::Lexer, types::PropertyType};

    use super::{SyntaxTree, UniversalSceneDescriptionFile};

    #[test]
    fn def() {
        let input = r#"#usda 1.0
                        def Xform "root" {
                            int[] faceVertexCounts = [3, 3]
                            int[] faceVertexIndices = [0, 1, 2, 0, 2, 3]
                            color3f[] primvars:displayColor = [(0, 0, 1)]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let usd = UniversalSceneDescriptionFile::new(&syntax_tree);

        // def が抽出されている
        assert_eq!(usd.definitions().len(), 1);

        // def のパラメータが抽出できている
        let definition = &usd.definitions()[0];
        assert_eq!(definition.name, "root");

        let PropertyType::FaceVertexCounts(data) = &definition.properties[0].property else {
            panic!()
        };
        assert_eq!(data, &[3, 3]);

        let PropertyType::FaceVertexIndicies(data) = &definition.properties[1].property else {
            panic!()
        };
        assert_eq!(data, &[0, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn def_color3f() {
        let input = r#"#usda 1.0
                        def Xform "root" {
                            color3f[] primvars:displayColor = [(0, 0, 1)]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let usd = UniversalSceneDescriptionFile::new(&syntax_tree);

        // def が抽出されている
        assert_eq!(usd.definitions().len(), 1);

        // def のパラメータが抽出できている
        let definition = &usd.definitions()[0];
        assert_eq!(definition.name, "root");

        // assert_eq!(definition.properties[0].identifier, "primvars:displayColor");
    }

    #[test]
    fn def_int_array_property() {
        let input = r#"#usda 1.0
                        def Xform "root" {
                            int[] faceVertexCounts = [3, 3, 3]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let usd = UniversalSceneDescriptionFile::new(&syntax_tree);

        // def が抽出されている
        assert_eq!(usd.definitions().len(), 1);

        // def のパラメータが抽出できている
        let definition = &usd.definitions()[0];
        assert_eq!(definition.name, "root");

        assert_eq!(definition.properties[0].identifier, "faceVertexCounts");

        let PropertyType::FaceVertexCounts(data) = &definition.properties[0].property else {
            panic!()
        };
        assert_eq!(data, &[3, 3, 3]);
    }

    #[test]
    fn def_points_property() {
        let input = r#"#usda 1.0
                        def Xform "root" {
                            point3f[] points = [(-0.5, -0.5, 1.0), (0.5, -0.5, 1.0), (0.0, 0.5, 1.0)]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let usd = UniversalSceneDescriptionFile::new(&syntax_tree);

        // def が抽出されている
        assert_eq!(usd.definitions().len(), 1);

        // def のパラメータが抽出できている
        let definition = &usd.definitions()[0];
        assert_eq!(definition.name, "root");
        assert_eq!(definition.properties[0].identifier, "points");

        let PropertyType::Points(data) = &definition.properties[0].property else {
            panic!()
        };

        assert_eq!(
            data,
            &[[-0.5, -0.5, 1.0], [0.5, -0.5, 1.0], [0.0, 0.5, 1.0]]
        );
    }

    #[test]
    fn def_normals_property() {
        let input = r#"#usda 1.0
                        def Xform "sample" {
                            normal3f[] normals = [(0.09267433, 0.009209316, 0.99565387), (0.09267433, 0.009209316, 0.99565387)]
                }"#;
        let tokens = &Lexer::tokenize(input);
        let syntax_tree = SyntaxTree::new(tokens);
        let usd = UniversalSceneDescriptionFile::new(&syntax_tree);

        // def が抽出されている
        assert_eq!(usd.definitions().len(), 1);

        // def のパラメータが抽出できている
        let definition = &usd.definitions()[0];
        assert_eq!(definition.name, "sample");
        assert_eq!(definition.properties[0].identifier, "normals");

        let PropertyType::Normals(data) = &definition.properties[0].property else {
            panic!()
        };

        assert_eq!(
            data,
            &[
                [0.09267433, 0.009209316, 0.99565387],
                [0.09267433, 0.009209316, 0.99565387]
            ]
        );
    }
}
