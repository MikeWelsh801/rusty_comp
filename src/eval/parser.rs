use crate::syntax::syntax_nodes::{Expression, LetStatement, Statement};
use crate::syntax::tokens::TokenType::*;
use crate::syntax::{syntax_nodes::Program, tokens::Token};

use super::lex::Lexer;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            curr_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = Token::new(
            self.peek_token.token_type.clone(),
            self.peek_token.literal.clone(),
        );
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        loop {
            let statement: Option<Box<dyn Statement>> = match self.curr_token.token_type {
                LET_KEYWORD_TOKEN => self.parse_let_statement(),
                RETURN_KEYWORD_TOKEN => self.parse_return_statement(),
                IF_KEYWORD_TOKEN => self.parse_if_statement(),
                EOF | _ => break,
            };

            if let Some(s) = statement {
                program.add_statement(s);
            }
            self.next_token();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        self.next_token();
        let identifier = Token::new(
            self.curr_token.token_type.clone(),
            self.curr_token.literal.clone(),
        );
        self.next_token();
        if self.curr_token.token_type != EQUAL_TOKEN {
            self.parse_error("Missing equal sign from assignment.");
            return None;
        }

        self.next_token();
        match self.parse_expression() {
            Some(expression) => Some(Box::from(LetStatement::new(identifier, expression))),
            None => None,
        }
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        todo!()
    }

    fn parse_if_statement(&self) -> Option<Box<dyn Statement>> {
        todo!()
    }

    fn parse_error(&self, arg: &str) {
        todo!()
    }

    fn parse_expression(&self) -> Option<Box<dyn Expression>> {
        todo!()
    }
}
