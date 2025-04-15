use std::{env, fs};
use std::error::Error;
use std::path::Path;
use std::process::exit;
use riscv_assembler::assembler::{Assembler, hexdump};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Check for assembly file
    if args.len() != 2 {
        eprintln!("Usage: {} <asm_file>", args[0]);
        exit(1);
    }

    let asm_file = &args[1];

    // Set the output paths
    let asm_file_path = Path::new(&asm_file);
    let bin_out_path = asm_file_path.with_extension("bin");
    let hex_out_path = asm_file_path.with_extension("hex");

    // Assemble the file and generate the hexdump
    let mut assembler = Assembler::new();
    let bin_out = assembler.assemble(asm_file)?;
    let hex_out = hexdump::generate_hexdump(&bin_out);

    // Write the output files
    fs::write(&bin_out_path, &bin_out)?;
    fs::write(&hex_out_path, &hex_out)?;

    // Determine the output directory for printing and set a fallback
    let output_dir = bin_out_path.parent().unwrap_or(Path::new("."));
    println!("Wrote bin and hex files to: {}", output_dir.display());

    Ok(())
}
