mod tokenizer;

use std::env;
use tokenizer::tokenizer::{tokenize, TokenKind};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let valid_args = validate_args(args)?;
    match tokenize(&valid_args[1]) {
        Ok(tokens) => match tokens[0] {
            TokenKind::Number(_) => output(tokens),
            _ => Err("The first token is not number.".to_string()),
        },
        Err(_) => Err("Tokenize error".to_string()),
    }
}

fn validate_args(args: Vec<String>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err("The number of args is invalid.".to_string());
    }

    Ok(args)
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
                    _ => return Err("The token following + operator is not number.".to_string()),
                }
            }
            TokenKind::Minus => {
                tokens.next();
                match tokens.peek() {
                    Some(&TokenKind::Number(number)) => {
                        tokens.next();
                        println!("  sub rax, {:?}", *number);
                    }
                    _ => return Err("The token following - operator is not number.".to_string()),
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
