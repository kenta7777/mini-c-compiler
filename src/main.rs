use std::{char, env};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let valid_args = validate_args(args)?;
    output_assembly_codes(&valid_args[1])
}

fn validate_args(args: Vec<String>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err("The number of args is invalid.".to_string());
    }

    Ok(args)
}

// TODO: Add test
fn output_assembly_codes(input: &str) -> Result<(), String> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    output_add_or_sub_codes(input)
}

fn output_add_or_sub_codes(input: &str) -> Result<(), String> {
    let mut char_vec: Vec<char> = Vec::new();
    let mut chars = (*input).chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            chars.next();
            char_vec.push(c);
        } else {
            break;
        }
    }
    let num = char_vec.iter().collect::<String>();
    println!("  mov rax, {:?}", (*num).parse::<u32>().unwrap());

    while let Some(&c) = chars.peek() {
        match c {
            '+' => {
                chars.next();
                let mut num_char_vec: Vec<char> = Vec::new();
                while let Some(&num_char) = chars.peek() {
                    if num_char.is_ascii_digit() {
                        chars.next();
                        num_char_vec.push(num_char);
                    } else {
                        break;
                    }
                }
                let num = num_char_vec.iter().collect::<String>();
                println!("  add rax, {:?}", num.parse::<u32>().unwrap());
            }
            '-' => {
                chars.next();
                let mut num_char_vec: Vec<char> = Vec::new();
                while let Some(&num_char) = chars.peek() {
                    if num_char.is_ascii_digit() {
                        chars.next();
                        num_char_vec.push(num_char);
                    } else {
                        break;
                    }
                }
                let num = num_char_vec.iter().collect::<String>();
                println!("  sub rax, {:?}", num.parse::<u32>().unwrap());
            }
            _ => return Err("unexpected operator".to_string()),
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
