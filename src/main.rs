use std::env;
use std::error::Error;
use std::process::exit;

mod assembler;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Check for assembly file
    if args.len() != 2 {
        eprintln!("Usage: {} <asm_file>", args[0]);
        exit(1);
    }

    Ok(())
}
