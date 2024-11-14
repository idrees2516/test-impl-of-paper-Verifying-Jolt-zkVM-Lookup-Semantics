use crate::core::*;
use crate::utils::BitOps;

pub trait JoltInstruction {
    fn execute(&self, x: u64, y: u64, w: u8) -> u64;
}

pub struct AndInstruction;
impl JoltInstruction for AndInstruction {
    fn execute(&self, x: u64, y: u64, w: u8) -> u64 {
        x & y & ((1u64 << w) - 1)
    }
}

pub struct OrInstruction;
impl JoltInstruction for OrInstruction {
    fn execute(&self, x: u64, y: u64, w: u8) -> u64 {
        x | y & ((1u64 << w) - 1)
    }
}

pub struct XorInstruction;
impl JoltInstruction for XorInstruction {
    fn execute(&self, x: u64, y: u64, w: u8) -> u64 {
        x ^ y & ((1u64 << w) - 1)
    }
}

pub struct AddInstruction;
impl JoltInstruction for AddInstruction {
    fn execute(&self, x: u64, y: u64, w: u8) -> u64 {
        x.wrapping_add(y) & ((1u64 << w) - 1)
    }
}

// Add more instruction implementations here following the same pattern