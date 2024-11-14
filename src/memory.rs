use std::collections::HashMap;

#[derive(Debug)]
pub struct Memory {
    data: HashMap<u64, u64>,
    width: u8,
}

impl Memory {
    pub fn new(width: u8) -> Self {
        Memory {
            data: HashMap::new(),
            width,
        }
    }

    pub fn read(&self, address: u64) -> u64 {
        *self.data.get(&address).unwrap_or(&0)
    }

    pub fn write(&mut self, address: u64, value: u64) {
        self.data.insert(address, value & ((1u64 << self.width) - 1));
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}