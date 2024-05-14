use super::{
    ast::SyntaxNode,
    syntax_nodes::{Expression, Statement},
    syntax_tokens::Token,
};

#[derive(Debug)]
pub(crate) struct LetStatement {
    let_keyword: Token,
    identifier: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {}
impl SyntaxNode for LetStatement {
    fn token_literal(&self) -> String {
        self.let_keyword.literal.clone()
    }
}

#[derive(Debug)]
pub(crate) struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {}
impl SyntaxNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
