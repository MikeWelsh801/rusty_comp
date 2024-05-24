use super::{ast::SyntaxNode, syntax_nodes::Expression, syntax_tokens::Token};

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        let value = token.literal.clone();
        Identifier { token, value }
    }

    pub fn name(&self) -> &str {
        &self.value
    }
}

impl Expression for Identifier {}
impl SyntaxNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
