mod ast;
mod identifier;
mod let_statement;
mod program;
mod return_statement;
mod syntax_tokens;

pub(crate) mod tokens {
    pub(crate) use crate::syntax::syntax_tokens::identifier_lookup;
    pub use crate::syntax::syntax_tokens::{Token, TokenType};
}

pub mod syntax_nodes {
    pub use crate::syntax::{
        ast::{Expression, Statement, SyntaxNode},
        identifier::Identifier,
        let_statement::LetStatement,
        program::Program,
        return_statement::ReturnStatement,
    };
}
