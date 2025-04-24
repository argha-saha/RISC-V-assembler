pub struct Section {
    name: String,
    address: u32,
    data: Vec<u8>,
    align: u32
}

impl Section {
    pub fn new(name: &str, address: u32, align: u32) -> Self {
        Self {
            name: name.to_string(),
            address,
            data: Vec::new(),
            align
        }
    }
}