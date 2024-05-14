use super::ast::Statement;
use super::ast::SyntaxNode;

#[derive(Debug)]
pub(crate) struct Program {
    statements: Vec<Box<dyn Statement>>
}

impl SyntaxNode for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::from("");
        }
        self.statements[0].token_literal()
    }
}
