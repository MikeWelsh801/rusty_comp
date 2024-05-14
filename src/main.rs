use std::io::{self, Write};
use compiler::prelude::Lexer;

fn main() {
    // main repl
    println!("Welcome type a command!");

    let mut lexer = Lexer::new();

    loop {
        // print the command line
        print!(">> ");
        io::stdout().flush().unwrap();

        // take in user input unless there's an error
        let mut buf = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buf) {
            lexer.lex(&buf);
            lexer.print_tokens();
        } else {
            panic!("Oh no!!!");
        }
    }
}
