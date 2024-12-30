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
        return Err("The number of args is invalid.".to_string())
    }

    match args[1].parse::<i64>() {
        Ok(_) => Ok(args),
        Err(_) => Err("fails to parse args[1] to integer.".to_string())
    }
}

fn output_assembly_codes(number: i64) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {:?}", number);
    println!("  ret");
}
