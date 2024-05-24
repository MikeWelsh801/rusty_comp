use crate::syntax::syntax_nodes::{Expression, Statement, LetStatement, Identifier};
use crate::syntax::tokens::TokenType::{self, *};
use crate::syntax::{syntax_nodes::Program, tokens::Token};

use super::lex::Lexer;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            curr_token,
            peek_token,
            errors: vec![],
        }
    }

    /* HELPERS FOR CHECKING TOKENS */

    fn next_token(&mut self) {
        self.curr_token = Token::new(
            self.peek_token.token_type.clone(),
            self.peek_token.literal.clone(),
        );
        self.peek_token = self.lexer.next_token();
    }

    fn curr_token_is(&self, token: TokenType) -> bool {
        self.curr_token.token_type == token
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        self.peek_token.token_type == token
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "Expected next token to be {:?}, but got {:?}",
            t, self.peek_token.token_type
        ))
    }

    /* End of helpers  */

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        loop {
            let statement: Option<Box<dyn Statement>> = match self.curr_token.token_type {
                LET_KEYWORD_TOKEN => self.parse_let_statement(),
                RETURN_KEYWORD_TOKEN => self.parse_return_statement(),
                IF_KEYWORD_TOKEN => self.parse_if_statement(),
                EOF => break,
                _ => None,
            };
            if let Some(s) = statement {
                program.add_statement(s);
            }
            self.next_token();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expect_peek(IDENTIFIER_TOKEN) {
            return None;
        }
        let name = Token::new(
            self.curr_token.token_type.clone(),
            self.curr_token.literal.clone(),
        );

        if !self.expect_peek(EQUAL_TOKEN) {
            return None;
        }

        // do this for now
        while !self.curr_token_is(SEMI_COLON_TOKEN) {
            self.next_token();
        }

        // TODO: Implement expressin parsing
        // match self.parse_expression() {
        //     Some(expression) => Some(Box::from(LetStatement::new(name, expression))),
        //     None => None,
        // }

        let value = Token::new(
            self.curr_token.token_type.clone(),
            self.curr_token.literal.clone(),
        );

        Some(Box::from(LetStatement::new(name, Box::from(Identifier::new(value)))))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        todo!()
    }

    fn parse_if_statement(&self) -> Option<Box<dyn Statement>> {
        todo!()
    }

    fn parse_expression(&self) -> Option<Box<dyn Expression>> {
        match self.curr_token.token_type {
            INT_LITERAL_TOKEN => todo!(),
            EOF => todo!(),
            ILLEGAL => todo!(),
            PLUS_TOKEN => todo!(),
            MINUS_TOKEN => todo!(),
            BANG_TOKEN => todo!(),
            SLASH_TOKEN => todo!(),
            STAR_TOKEN => todo!(),
            LESS_THAN_TOKEN => todo!(),
            GREATER_THAN_TOKEN => todo!(),
            EQUAL_EQUAL_TOKEN => todo!(),
            BANG_EQUAL_TOKEN => todo!(),
            OPEN_PAREN_TOKEN => todo!(),
            CLOSE_PAREN_TOKEN => todo!(),
            OPEN_BRACKET_TOKEN => todo!(),
            CLOSE_BRACKET_TOKEN => todo!(),
            COMMA_TOKEN => todo!(),
            SEMI_COLON_TOKEN => todo!(),
            EQUAL_TOKEN => todo!(),
            LET_KEYWORD_TOKEN => todo!(),
            FUNC_KEYWORD_TOKEN => todo!(),
            RETURN_KEYWORD_TOKEN => todo!(),
            IF_KEYWORD_TOKEN => todo!(),
            ELSE_KEYWORD_TOKEN => todo!(),
            IDENTIFIER_TOKEN => todo!(),
        }
    }
}
