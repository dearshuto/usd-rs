use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("#usda")]
    Usda,

    #[token("{")]
    BraceL,

    #[token("}")]
    BraceR,

    #[token("[")]
    BracketL,

    #[token("]")]
    BracketR,

    #[token("(")]
    ParenthesesL,

    #[token(")")]
    ParenthesesR,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("=")]
    Equal,

    #[regex(r"[+-]?[0-9]+(?:\.[0-9]+)?")]
    Number,

    #[token("bool")]
    Bool,

    #[token("color3f")]
    Color3f,

    #[token("color3f[]")]
    Color3fArray,

    #[token("float3")]
    Float3f,

    #[token("float3[]")]
    Float3fArray,

    #[token("double3")]
    Double3f,

    #[token("int[]")]
    IntArray,

    #[token("point3f")]
    Point3f,

    #[token("point3f[]")]
    Point3fArray,

    #[token("normal3f[]")]
    Normal3fArray,

    #[token("uniform")]
    Uniform,

    #[token("null")]
    NullValue,

    #[token("def")]
    Def,

    #[token("Xform")]
    Xform,

    #[token("xformOp")]
    XformOp,

    #[token("xformOpOrder")]
    XformOpOrder,

    #[token("translate")]
    Translate,

    #[token("token")]
    Token,

    #[regex("\"[a-zA-Z_]+\"")]
    Text,

    #[token("true")]
    True,

    #[token("false")]
    False,

    Identifier(String),
    NumberValue(u32),
}
