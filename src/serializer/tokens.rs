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

    // 数値配列を抽出
    // マッチする例：(0, 1, -2, -4, 0.5, -0.1)
    //
    // 正規表現は <数字> の後に ", <数字>"を繰り返すような構文
    // (<整数>[, <数字>][, <数字>][, <数字>]...)
    #[regex(r"\([-]*\d+(?:\.\d+)?(,\s*[-]*\d+(?:\.\d+)?)*\)")]
    NumberArray,

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

    #[regex("[a-zA-Z]+")]
    IdentifierTentative,

    #[token("true")]
    True,

    #[token("false")]
    False,

    Identifier(String),
    NumberValue(u32),
}
