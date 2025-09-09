pub enum Tokens {
    Left,
    Right,
    Plus,
    Minus,
    LParen,
    RParen,
    Read,
    Write
}

pub fn tokenize(program: &str) -> Vec<Tokens> {
    let mut result: Vec<Tokens> = vec![];

    for symbol in program.chars() {
        let token: Tokens = match symbol {
            '<' => Tokens::Left,
            '>' => Tokens::Right,
            '+' => Tokens::Plus,
            '-' => Tokens::Minus,
            '[' => Tokens::LParen,
            ']' => Tokens::RParen,
            ',' => Tokens::Read,
            '.' => Tokens::Write,
            _ => continue
        };

        result.push(token);
    }

    result
}