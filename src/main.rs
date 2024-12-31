use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let valid_args = validate_args(args)?;
    let i64_number = valid_args[1].parse::<i64>().unwrap();
    output_assembly_codes(i64_number);

    Ok(())
}

fn validate_args(args: Vec<String>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err("The number of args is invalid.".to_string());
    }

    match args[1].parse::<i64>() {
        Ok(_) => Ok(args),
        Err(_) => Err("fails to parse args[1] to integer.".to_string()),
    }
}
// TODO: Add test
fn output_assembly_codes(number: i64) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {:?}", number);
    println!("  ret");
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

    #[test]
    fn validate_args_returns_error_when_number_of_args_is_invalid() {
        let mut args = Vec::new();
        args.push("target/debug/mini-c-compiler".to_string());
        args.push(5.to_string());
        args.push("invalid arg".to_string());

        assert_eq!(
            validate_args(args.clone()),
            Err("The number of args is invalid.".to_string())
        );
    }

    #[test]
    fn validate_args_returns_error_when_args_1th_is_not_i64() {
        let mut args = Vec::new();
        args.push("target/debug/mini-c-compiler".to_string());
        args.push("invalid arg".to_string());

        assert_eq!(
            validate_args(args.clone()),
            Err("fails to parse args[1] to integer.".to_string())
        );
    }
}
