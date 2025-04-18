//! Provides functionality for handling pseudo-instructions

use phf::phf_set;
use crate::assembler::AssemblerError;
use crate::assembler::parser::parse_immediate;

pub struct TranslatedInstruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<String>,
}

static PSEUDO_INSTRUCTIONS: phf::Set<&'static str> = phf_set! {
    "nop",
    "li",
    "mv",
    "not",
    "neg",
    "negw",
    "sext.w",
    "seqz",
    "snez",
    "sltz",
    "sgtz",
    "beqz",
    "bnez",
    "blez",
    "bgez",
    "bltz",
    "bgtz",
    "bgt",
    "ble",
    "bgtu",
    "bleu",
    "j",
    "jr",
    "ret"
};

pub struct PseudoInstructions;

impl PseudoInstructions {
    // Check if a mnemonic is a pseudo-instruction
    pub fn is_pseudo_instruction(mnemonic: &str) -> bool {
        if PSEUDO_INSTRUCTIONS.contains(mnemonic) {
            true
        } else {
            false
        }
    }
    
    // Translate a pseudo-instruction into one or more base instructions
    pub fn expand<'a>(
        mnemonic: &'a str, 
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        match mnemonic {
            "nop" => Self::translate_nop(operands),
            "li" => Self::translate_li(operands),
            "mv" => Self::translate_mv(operands),
            "not" => Self::translate_not(operands),
            "neg" => Self::translate_neg(operands),
            "negw" => Self::translate_negw(operands),
            "sext.w" => Self::translate_sextw(operands),
            "seqz" => Self::translate_seqz(operands),
            "snez" => Self::translate_snez(operands),
            "sltz" => Self::translate_sltz(operands),
            "sgtz" => Self::translate_sgtz(operands),
            "beqz" => Self::translate_beqz(operands),
            "bnez" => Self::translate_bnez(operands),
            "blez" => Self::translate_blez(operands),
            "bgez" => Self::translate_bgez(operands),
            "bltz" => Self::translate_bltz(operands),
            "bgtz" => Self::translate_bgtz(operands),
            "bgt" => Self::translate_bgt(operands),
            "ble" => Self::translate_ble(operands),
            "bgtu" => Self::translate_bgtu(operands),
            "bleu" => Self::translate_bleu(operands),
            "j" => Self::translate_j(operands),
            "jr" => Self::translate_jr(operands),
            "ret" => Self::translate_ret(operands),
            _ => Err(AssemblerError::InvalidInstruction(format!(
                "Unknown pseudo-instruction: {}", mnemonic
            )))
        }
    }
    
    // No operation
    // nop => addi x0, x0, 0
    fn translate_nop<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("nop", operands, 0)?;
        
        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addi",
                operands: vec![
                    "x0".to_string(),  // x0
                    "x0".to_string(),  // x0
                    "0".to_string(),   // 0
                ],
            }
        ])
    }

    // Load immediate
    // li rd, immediate => lui + addi
    fn translate_li<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("li", operands,2)?;
        let rd = operands[0];
        let imm = parse_immediate(operands[1])?;

        // Case 1: immediate fits within the 12-bit range
        if (-2048..=2047).contains(&imm) {
            return Ok(vec![
                TranslatedInstruction {
                    mnemonic: "addi",
                    operands: vec![
                        rd.to_string(),
                        "x0".to_string(),
                        imm.to_string()
                    ]
                }
            ])
        }

        // Case 2: immediate requires lui + addi
        let imm_upper = ((imm + 0x800) >> 12) as i32;  // Upper 20 bits
        let imm_lower = imm - (imm_upper << 12);       // Lower 12 bits signed

        let mut expanded = vec![
            TranslatedInstruction {
                mnemonic: "lui",
                operands: vec![
                    rd.to_string(),        // rd
                    imm_upper.to_string()  // imm[31:12]
                ]
            }
        ];

        if imm_lower != 0 {
            expanded.push(
                TranslatedInstruction {
                    mnemonic: "addi",
                    operands: vec![
                        rd.to_string(),        // rd
                        rd.to_string(),        // rd
                        imm_lower.to_string()  // imm[11:0]
                    ]
                }
            )
        }

        Ok(expanded)
    }

    // Copy register
    // mv rd, rs => addi rd, rs, 0
    fn translate_mv<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("mv", operands, 2)?;
        
        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addi",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "0".to_string(),          // 0
                ],
            }
        ])
    }

    // One's complement
    // not rd, rs => xori rd, rs, -1
    fn translate_not<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("not", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "xori",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "-1".to_string()          // -1
                ]
            }
        ])
    }

    // Two's complement
    // neg rd, rs => sub rd, x0, rs
    fn translate_neg<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("neg", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sub",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string(),  // rs
                ]
            }
        ])
    }

    // Two's complement word
    // negw rd, rs => subw rd, x0, rs
    fn translate_negw<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("negw", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "subw",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string(),  // rs
                ]
            }
        ])
    }

    // Sign extend word
    // sext.w rd, rs => addiw rd, rs, 0
    fn translate_sextw<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("sext.w", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addiw",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "0".to_string()           // 0
                ]
            }
        ])
    }

    // Set if == zero
    // seqz rd, rs => sltiu rd, rs, 1
    fn translate_seqz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("seqz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sltiu",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "1".to_string()           // 1
                ]
            }
        ])
    }

    // Set if != zero
    // snez rd, rs => sltu rd, x0, rs
    fn translate_snez<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("snez", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sltu",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // rs
                ]
            }
        ])
    }

    // Set if < zero
    // sltz rd, rs => slt rd, rs, x0
    fn translate_sltz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("sltz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "slt",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "x0".to_string()          // x0
                ]
            }
        ])
    }

    // Set if > zero
    // sgtz rd, rs => slt rd, x0, rs
    fn translate_sgtz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("sgtz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "slt",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // rs
                ]
            }
        ])
    }

    // Branch if == zero
    // beqz rs, offset => beq rs, x0, offset
    fn translate_beqz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("beqz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "beq",
                operands: vec![
                    operands[0].to_string(),  // rs
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if != zero
    // bnez rs, offset => bne rs, x0, offset
    fn translate_bnez<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bnez", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bne",
                operands: vec![
                    operands[0].to_string(),  // rs
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if <= zero
    // blez rs, offset => bge x0, rs, offset
    fn translate_blez<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("blez", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bge",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // rs
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if >= zero
    // bgez rs, offset => bge rs, x0, offset
    fn translate_bgez<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bgez", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bge",
                operands: vec![
                    operands[0].to_string(),  // rs
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if < zero
    // bltz rs, offset => bge rs, x0, offset
    fn translate_bltz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bltz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "blt",
                operands: vec![
                    operands[0].to_string(),  // rs
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if > zero
    // bgtz rs, offset => blt x0, rs, offset
    fn translate_bgtz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bgtz", operands, 2)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "blt",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // rs
                    operands[1].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if >
    // bgt rs, rt, offset => blt rt, rs, offset
    fn translate_bgt<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bgt", operands, 3)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "blt",
                operands: vec![
                    operands[1].to_string(),  // rt
                    operands[0].to_string(),  // rs
                    operands[2].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if <=
    // ble rs, rt, offset => bge rt, rs, offset
    fn translate_ble<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("ble", operands, 3)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bge",
                operands: vec![
                    operands[1].to_string(),  // rt
                    operands[0].to_string(),  // rs
                    operands[2].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if >, unsigned
    // bgtu rs, rt, offset => bltu rt, rs, offset
    fn translate_bgtu<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bgtu", operands, 3)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bltu",
                operands: vec![
                    operands[1].to_string(),  // rt
                    operands[0].to_string(),  // rs
                    operands[2].to_string()   // offset
                ]
            }
        ])
    }

    // Branch if <=, unsigned
    // bleu rs, rt, offset => bgeu rt, rs, offset
    fn translate_bleu<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("bleu", operands, 3)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "bgeu",
                operands: vec![
                    operands[1].to_string(),  // rt
                    operands[0].to_string(),  // rs
                    operands[2].to_string()   // offset
                ]
            }
        ])
    }

    // Jump
    // j offset => jal x0, offset
    fn translate_j<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("j", operands, 1)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jal",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // offset
                ]
            }
        ])
    }

    // Jump register
    // jr rs => jalr x0, rs, 0
    fn translate_jr<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("jr", operands, 1)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jalr",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // rs
                    "0".to_string()           // 0
                ]
            }
        ])
    }

    // Return from subroutine
    // ret => jalr x0, x1, 0
    fn translate_ret<'a>(operands: &[&str]) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("ret", operands, 0)?;

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jalr",
                operands: vec![
                    "x0".to_string(),  // x0
                    "x1".to_string(),  // x1
                    "0".to_string()    // 0
                ]
            }
        ])
    }
}

// Validate operands
fn check_operands(
    name: &str,
    operands: &[&str],
    expected_len: usize
) -> Result<(), AssemblerError> {
    if operands.len() != expected_len {
        return Err(AssemblerError::InvalidOperand(format!(
            "Expected {} operands for {} but received {}",
            expected_len,
            name,
            operands.len()
        )))
    }

    Ok(())
}