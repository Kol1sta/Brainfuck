use std::{
    env,
    fs,
    io,
    path::PathBuf
};
use crate::interpreter::{
    lexer::tokenize,
    parser::parse
};

pub fn handle_commands(args: Vec<String>) -> io::Result<()> {
    match args[1].as_str() {
        "--version" | "-v" | "--v" => {
            println!("Brainfuck interpreter version: {}", env::var("VERSION").unwrap_or("Unknown".to_string()));
        },
        "--help" | "-h" | "--h" => {
            println!("Usage: [interpreter path] [input]");
            println!("\nOptions: ");
            println!("\t-h, --help:    display this message");
            println!("\t-v, --version: display your version of brainfuck interpreter");
            println!("\nTo run a file, call the interpreter with your .bf file");
            println!("or run the interpreter without options to start interactive mode");
        },
        file_name if file_name.starts_with("-") => {
            eprintln!("Unknown option: {}", file_name);
        },
        file_name => {
            let file_path = PathBuf::from(file_name);
            run_program(file_path)?;
        }
    }

    Ok(())
}

fn run_program(file_path: PathBuf) -> io::Result<()> {
    let file = fs::read(file_path)?;
    let program: String = String::from_utf8_lossy(&file).to_string();
    parse(tokenize(program.as_str()))?;

    Ok(())
}
