use crate::prelude::Token;
use crate::prelude::TokenType::*;
use crate::syntax::tokens::identifier_lookup;

pub struct Lexer {
    input: String,
    position: usize,
    ch: char,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: String::from(""),
            position: 0,
            ch: '\0',
        }
    }

    pub fn lex(&mut self, input: &str) {
        self.input = String::from(input);
        self.ch = input.as_bytes()[0] as char;
        self.position = 0;
    }

    fn peek(&self) -> char {
        if self.position >= self.input.len() - 1 {
            return '\0';
        }

        self.input.as_bytes()[self.position + 1] as char
    }

    fn read_char(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.position] as char;
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut word = self.ch.to_string();
        while self.next_token_in_identifier() {
            self.read_char();
            word.push(self.ch);
        }
        Token::new(identifier_lookup(&word[..]), word)
    }

    fn read_number(&mut self) -> Token {
        let mut word = self.ch.to_string();
        while self.peek().is_numeric() {
            self.read_char();
            word.push(self.ch);
        }

        Token::new(INT_LITERAL_TOKEN, word)
    }

    fn eat_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let token = match self.ch {
            '+' => Token::new(PLUS_TOKEN, "+".to_string()),
            '-' => Token::new(MINUS_TOKEN, "-".to_string()),
            '*' => Token::new(STAR_TOKEN, "*".to_string()),
            '/' => Token::new(SLASH_TOKEN, "/".to_string()),

            '<' => Token::new(LESS_THAN_TOKEN, "<".to_string()),
            '>' => Token::new(GREATER_THAN_TOKEN, ">".to_string()),

            '(' => Token::new(OPEN_PAREN_TOKEN, "(".to_string()),
            ')' => Token::new(CLOSE_PAREN_TOKEN, ")".to_string()),
            '{' => Token::new(OPEN_BRACKET_TOKEN, "{".to_string()),
            '}' => Token::new(CLOSE_BRACKET_TOKEN, "}".to_string()),

            ',' => Token::new(COMMA_TOKEN, ",".to_string()),
            ';' => Token::new(SEMI_COLON_TOKEN, ";".to_string()),
            '\0' => Token::new(EOF, "\0".to_string()),

            '=' => {
                if self.peek() == '=' {
                    self.read_char(); // consume a token
                    Token::new(EQUAL_EQUAL_TOKEN, "==".to_string())
                } else {
                    Token::new(EQUAL_TOKEN, "=".to_string())
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.read_char(); // consume a token
                    Token::new(BANG_EQUAL_TOKEN, "!=".to_string())
                } else {
                    Token::new(BANG_TOKEN, "!".to_string())
                }
            }
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    self.read_identifier()
                } else if self.ch.is_numeric() {
                    self.read_number()
                } else {
                    Token::new(ILLEGAL, self.ch.to_string())
                }
            }
        };
        self.read_char();
        token
    }

    fn next_token_in_identifier(&self) -> bool {
        let c = self.peek();
        c.is_alphanumeric() || c.is_numeric() || c == '_'
    }
}
