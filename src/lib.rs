mod lexer;
mod syntax;

pub mod prelude {
    pub use crate::lexer::Lexer;
    pub(crate) use crate::syntax::tokens::Token;
    pub use crate::syntax::tokens::TokenType;
}
