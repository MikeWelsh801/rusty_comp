mod syntax_tokens;
mod ast;
mod program;
mod let_statement;

pub(crate) mod tokens {
    pub use crate::syntax::syntax_tokens::Token;
    pub use crate::syntax::syntax_tokens::TokenType;
    pub(crate) use crate::syntax::syntax_tokens::identifier_lookup;
}

pub(crate) mod syntax_nodes {
    pub(crate) use crate::syntax::program::Program;
    pub(crate) use crate::syntax::ast::Statement;
    pub(crate) use crate::syntax::ast::Expression;
    pub(crate) use crate::syntax::let_statement::LetStatement;
}

