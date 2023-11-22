mod analyzer;
mod syntax_tree;
mod tokens;
mod types;
mod universal_scene_description_file;

use analyzer::Lexer;
use syntax_tree::SyntaxTree;
pub use tokens::{Token, CHAR_TOKENS, KEYWORD_TOKENS};
pub use types::UpAxis;
pub use universal_scene_description_file::{Header, UniversalSceneDescriptionFile};

pub fn from_str(str: &str) -> Result<UniversalSceneDescriptionFile, ()> {
    let tokens = Lexer::tokenize(str);
    let syntax_tree = SyntaxTree::new(&tokens);
    let usd = UniversalSceneDescriptionFile::new(&syntax_tree);
    Ok(usd)
}
