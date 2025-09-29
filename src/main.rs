use std::{
    io,
    env
};
use crate::{
    commands::handle_commands,
    interactive::interactive_mode
};

pub mod interpreter;
pub mod interactive;
pub mod commands;

fn main() -> io::Result<()> {
    if let Err(e) = dotenvy::dotenv_override() {
        eprintln!("Warning, failed to load .env file: {}", e);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        handle_commands(args)?;
    } else {
        interactive_mode()?;
    }

    Ok(())
}
