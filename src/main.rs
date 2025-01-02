use std::{char, env, iter::Peekable, str::Chars};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let valid_args = validate_args(args)?;
    match tokenize(&valid_args[1]) {
        Ok(tokens) => output(tokens),
        Err(_) => Err("Tokenize error".to_string()),
    }
}

fn validate_args(args: Vec<String>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err("The number of args is invalid.".to_string());
    }

    Ok(args)
}

#[derive(Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Number(u32),
}

pub enum TokenizeError {
    UnexpectedToken
}

pub fn tokenize(input: &String) -> Result<Vec<TokenKind>, TokenizeError> {
    let mut tokens: Vec<TokenKind> = Vec::new();
    let mut chars = (*input).chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '+' => {
                chars.next();
                tokens.push(TokenKind::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(TokenKind::Minus);
            }
            '0'..'9' => match tokenize_number(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
            _ => return Err(TokenizeError::UnexpectedToken),
        }
    }

    Ok(tokens)
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<TokenKind, TokenizeError> {
    let mut char_vec: Vec<char> = Vec::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            chars.next();
            char_vec.push(c);
        } else {
            break;
        }
    }
    let number = char_vec.iter().collect::<String>().parse::<u32>().unwrap();

    Ok(TokenKind::Number(number))
}

fn output(tokens: Vec<TokenKind>) -> Result<(), String> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut tokens = tokens.iter().peekable();
    while let Some(&token) = tokens.peek() {
        match *token {
            TokenKind::Plus => {
                tokens.next();
                match tokens.peek() {
                    Some(&TokenKind::Number(number)) => {
                        tokens.next();
                        println!("  add rax, {:?}", *number);
                    }
                    _ => return Err("The token following + operator is not a number".to_string()),
                }
            }
            TokenKind::Minus => {
                tokens.next();
                match tokens.peek() {
                    Some(&TokenKind::Number(number)) => {
                        tokens.next();
                        println!("  sub rax, {:?}", *number);
                    }
                    _ => return Err("The token following - operator is not a number".to_string()),
                }
            }
            TokenKind::Number(number) => {
                tokens.next();
                println!("  mov rax, {:?}", number);
            }
        }
    }

    println!("  ret");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::validate_args;

    #[test]
    fn validate_args_validates_command_line_args() {
        let mut args = Vec::new();
        args.push("target/debug/mini-c-compiler".to_string());
        args.push(5.to_string());

        assert_eq!(validate_args(args.clone()), Ok(args));
    }
}
