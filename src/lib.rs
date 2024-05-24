mod eval;
mod syntax;

pub mod prelude {
    pub use crate::eval::{lex::Lexer, parse::Parser};
    pub use crate::syntax::{
        syntax_nodes::*,
        tokens::{Token, TokenType},
    };
}

use prelude::Lexer;
use prelude::TokenType::*;

pub fn print_tokens(lexer: &mut Lexer) {
    let mut token = lexer.next_token();
    while token.token_type != EOF {
        println!("{:?}", token);
        token = lexer.next_token();
    }
}
