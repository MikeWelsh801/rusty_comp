mod lexer;
mod syntax;

pub mod prelude {
    pub use crate::lexer::Lexer;
    pub use crate::syntax::tokens::Token;
    pub use crate::syntax::tokens::TokenType;
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
