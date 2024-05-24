mod syntax_tokens;
mod ast;
mod program;
mod let_statement;
mod identifier;

pub(crate) mod tokens {
    pub use crate::syntax::syntax_tokens::Token;
    pub use crate::syntax::syntax_tokens::TokenType;
    pub(crate) use crate::syntax::syntax_tokens::identifier_lookup;
}

pub mod syntax_nodes {
    pub use crate::syntax::program::Program;
    pub use crate::syntax::ast::Statement;
    pub use crate::syntax::ast::Expression;
    pub use crate::syntax::let_statement::LetStatement;
    pub use crate::syntax::identifier::Identifier;
}

