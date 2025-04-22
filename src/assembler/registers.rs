use phf::phf_map;

pub(crate) const ABI_NAME_REGISTERS: phf::Map<&'static str, u32> = phf_map! {
    "zero" => 0,  // Zero constant
    "ra" => 1,    // Return address
    "sp" => 2,    // Stack pointer
    "gp" => 3,    // Global pointer
    "tp" => 4,    // Thread pointer
    "t0" => 5,    // Temporary
    "t1" => 6,    // Temporary
    "t2" => 7,    // Temporary
    "fp" => 8,    // Frame pointer
    "s0" => 8,    // Saved register
    "s1" => 9,    // Saved register
    "a0" => 10,   // Fn args/return values
    "a1" => 11,   // Fn args
    "a2" => 12,   // Fn args
    "a3" => 13,   // Fn args
    "a4" => 14,   // Fn args
    "a5" => 15,   // Fn args
    "a6" => 16,   // Fn args
    "a7" => 17,   // Fn args
    "s2" => 18,   // Saved register
    "s3" => 19,   // Saved register
    "s4" => 20,   // Saved register
    "s5" => 21,   // Saved register
    "s6" => 22,   // Saved register
    "s7" => 23,   // Saved register
    "s8" => 24,   // Saved register
    "s9" => 25,   // Saved register
    "s10" => 26,  // Saved register
    "s11" => 27,  // Saved register
    "t3" => 28,   // Temporary
    "t4" => 29,   // Temporary
    "t5" => 30,   // Temporary
    "t6" => 31,   // Temporary
};