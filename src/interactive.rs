use std::io;
use crate::interpreter::{
    lexer::tokenize,
    parser::parse
};

pub fn interactive_mode() -> io::Result<()> {
    let mut program: String = String::new();
    let mut input: String = String::new();

    println!("Welcome to brainfuck interactive mode! For getting help run interpreter with --help flag");
    println!("Commands:");
    println!("\trun: run program");
    println!("\tclear: clear context of your program");
    println!("\texit: close program");
    println!("Your program:");

    loop {
        input.clear();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read message: {}", e);
        }

        program.push_str(&input);

        match input.trim() {
            "run" => {
                parse(tokenize(program.as_str()))?
            },
            "clear" => {
                program.clear();
            },
            "exit" => {
                break;
            },
            _ => {}
        }
    }

    Ok(())
}
