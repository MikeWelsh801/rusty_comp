mod syntax_tokens;

pub(crate) mod tokens {
    pub(crate) use crate::syntax::syntax_tokens::Token;
    pub use crate::syntax::syntax_tokens::TokenType;
}
