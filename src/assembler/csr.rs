use phf::phf_map;

pub static CSR_ADDRESSES: phf::Map<&'static str, u32> = phf_map! {
    "mstatus" => 0x300,
    "misa" => 0x301,
    "mie" => 0x304,
    "mtvec" => 0x305,
    "mtvt" => 0x307,
    "mstatush" => 0x310
};