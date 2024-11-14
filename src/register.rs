use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegisterFile {
    general_purpose: Vec<u64>,
    vector_registers: Vec<VectorRegister>,
    width: u8,
}

#[derive(Debug, Clone)]
pub struct VectorRegister {
    elements: Vec<u64>,
    width: u8,
}

impl RegisterFile {
    pub fn new(num_gp: usize, num_vector: usize, width: u8) -> Self {
        RegisterFile {
            general_purpose: vec![0; num_gp],
            vector_registers: vec![VectorRegister::new(width, 4); num_vector],
            width,
        }
    }

    pub fn read_gp(&self, index: usize) -> Option<u64> {
        self.general_purpose.get(index).copied()
    }

    pub fn write_gp(&mut self, index: usize, value: u64) -> bool {
        if let Some(reg) = self.general_purpose.get_mut(index) {
            *reg = value & ((1u64 << self.width) - 1);
            true
        } else {
            false
        }
    }
}

impl VectorRegister {
    pub fn new(width: u8, size: usize) -> Self {
        VectorRegister {
            elements: vec![0; size],
            width,
        }
    }
}