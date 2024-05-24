use std::fmt;

pub trait SyntaxNode {
    fn token_literal(&self) -> String;
}

pub trait Statement: SyntaxNode + fmt::Debug {
    fn statement_node(&self) {
        println!("Statement: {{ {} }}", self.token_literal());
    }
}

pub trait Expression: SyntaxNode + fmt::Debug {
    fn expression_node(&self) {
        println!("Expression: {{ {} }}", self.token_literal());
    }
}

