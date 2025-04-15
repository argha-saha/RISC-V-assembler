# RISC-V Assembler
An assembler for the RISC-V instruction set, written in Rust.

## Features
- Supports RV32I base integer instructions
- Supports ABI name registers (e.g. a0-a7, t0-t6, etc.)
- Supports labels for b-type, u-type, and j-type instructions
- Generates a binary file (`.bin`) containing machine code
- Generates a hexdump file (`.hex`)

## Next Steps
- Add support for .text and .data segments
- Implement pseudo instructions (e.g. li, beqz, j)
- Implement RV32M multiply extension instructions
- Implement RV32A atomic extension instructions

## References
[RISC-V Reference](https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf)