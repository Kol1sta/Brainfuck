use std::{
    io,
    env
};
use crate::commands::handle_commands;

pub mod lexer;
pub mod parser;
pub mod interact_mode;
pub mod commands;

fn main() -> io::Result<()> {
    if let Err(e) = dotenvy::dotenv_override() {
        eprintln!("Warning, failed to load .env file: {}", e);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        handle_commands(args)?;
    } else {
        // Interactive mode
    }

    Ok(())
}
