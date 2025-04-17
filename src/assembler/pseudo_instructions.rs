//! Provides functionality for handling pseudo-instructions

use phf::phf_set;
use crate::assembler::AssemblerError;

pub struct TranslatedInstruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<String>,
}

static PSEUDO_INSTRUCTIONS: phf::Set<&'static str> = phf_set! {
    "nop",
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
            "j" => Self::translate_j(operands),
            "jr" => Self::translate_jr(operands),
            "ret" => Self::translate_ret(operands),
            _ => Err(AssemblerError::InvalidInstruction(format!(
                "Unknown pseudo-instruction: {}", mnemonic
            )))
        }
    }
    
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

    // j offset => jal x0, offset
    fn translate_j<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("j", operands, 2)?;

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

    // jr rs => jalr x0, rs, 0
    fn translate_jr<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("jr", operands, 2)?;

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

    // ret => jalr x0, x1, 0
    fn translate_ret<'a>(operands: &[&str]) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        check_operands("ret", operands, 2)?;

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