# RISC-V Assembler
An assembler for the RISC-V instruction set, written in Rust.

## Features
- Supports RV32I/RV32M/RV64I instructions
- Supports ABI name registers (e.g. a0-a7, t0-t6, etc.)
- Supports labels for b-type, u-type, and j-type instructions
- Generates a binary file (`.bin`) containing machine code
- Generates a hexdump file (`.hex`)

## Instruction Support
- RV32I: all r-type, i-type, s-type, b-type, u-type, and j-type (excludes atomics, fence, wfi, u/s/m ret)
- RV64I: addw, subw, sllw, srlw, sraw, addiw, slliw, srliw, sraiw, lwu, ld, sd
- Pseudo-instructions: la, lb, lh, lw, ld, sb, sh, sw, sd, nop, li, mv, not, neg, negw, sext.w, seqz, snez, sltz, sgtz, 
beqz, bnez, bltz, bgtz, bgt, ble, bgtu, bleu, j, jal, jr, jalr, ret, call, tail

## Next Steps
- Add support for .text and .data segments
- Add the rest of the instructions for RV32 such as fence
- Implement RV32A atomic extension instructions

## References
[RISC-V Reference](https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf)  
[RISC-V ISA Pages](https://msyksphinz-self.github.io/riscv-isadoc/html/rv64i.html)  
[Control and Status Registers](https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/priv-csrs.html)  