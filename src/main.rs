use std::io;
use crate::lexer::tokenize;
use crate::parser::parse;

pub mod lexer;
pub mod parser;

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    println!("Brainfuck compiler v1.0.0. Enter program:");
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read program");

        if input.trim() == "exit" {
            break;
        }

        let tokens = tokenize(&input);
        parse(&tokens);
    }

    println!("Enter any symbol to close program");
    io::stdin().read_line(&mut input).unwrap();

    Ok(())
}