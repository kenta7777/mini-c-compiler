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
fn output_assembly_codes(expr: &String) -> Result<(), String> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let (head, tail) = (*expr).split_at(1);
    println!("  mov rax, {:?}", (*head).parse::<u32>().unwrap());
    output_add_or_sub_ops(tail)
}

fn output_add_or_sub_ops(str: &str) -> Result<(), String> {
    let mut chars = (*str).chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '+' => {
                chars.next();
                let mut num_char_vec: Vec<char> = Vec::new();
                while let Some(&num_char) = chars.peek() {
                    if let Some(_) = num_char.to_digit(10) {
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
                    if let Some(_) = num_char.to_digit(10) {
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
