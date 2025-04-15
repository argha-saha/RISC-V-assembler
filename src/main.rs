use std::{env, fs};
use std::error::Error;
use std::path::Path;
use std::process::exit;
use riscv_assembler::assembler::Assembler;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Check for assembly file
    if args.len() != 2 {
        eprintln!("Usage: {} <asm_file>", args[0]);
        exit(1);
    }

    let asm_file = &args[1];
    let output_path = Path::new(asm_file)
        .with_extension("bin");

    let mut assembler = Assembler::new();
    let bin = assembler.assemble(asm_file)?;

    // Write the output file
    fs::write(&output_path, &bin)?;
    println!("Generated binary file: {}", output_path.display());

    Ok(())
}
