use super::ast::Statement;
use super::ast::SyntaxNode;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl SyntaxNode for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::from("");
        }
        self.statements[0].token_literal()
    }
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    pub(crate) fn add_statement(&mut self, s: Box<dyn Statement>) {
        self.statements.push(s);
    }
}
