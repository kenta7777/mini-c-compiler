use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let valid_args = validate_args(args)?;
    //TODO: Calculating numbers with 2 or more digits
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
    let mut chars = (*str).chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => match chars.next() {
                Some(n) => {
                    if let Some(num) = n.to_digit(10) {
                        println!("  add rax, {:?}", num)
                    } else {
                        return Err("The integer for addition is invalid".to_string());
                    }
                }
                None => return Err("The integer for addition does not exist.".to_string()),
            },
            '-' => match chars.next() {
                Some(n) => {
                    if let Some(num) = n.to_digit(10) {
                        println!("  sub rax, {:?}", num)
                    } else {
                        return Err("The integer for subtraction is invalid".to_string());
                    }
                }
                None => return Err("The integer for subtraction does not exist.".to_string()),
            },
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
