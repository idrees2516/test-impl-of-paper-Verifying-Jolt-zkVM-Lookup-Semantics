use crate::register::RegisterFile;
use crate::memory::Memory;
use crate::instructions::JoltInstruction;

pub struct ExecutionContext {
    pub registers: RegisterFile,
    pub memory: Memory,
    pub pc: usize,
}

impl ExecutionContext {
    pub fn new(width: u8) -> Self {
        ExecutionContext {
            registers: RegisterFile::new(32, 8, width),
            memory: Memory::new(width),
            pc: 0,
        }
    }

    pub fn execute_instruction<T: JoltInstruction>(&mut self, instruction: &T, rs1: usize, rs2: usize, rd: usize, w: u8) -> bool {
        if let (Some(x), Some(y)) = (self.registers.read_gp(rs1), self.registers.read_gp(rs2)) {
            let result = instruction.execute(x, y, w);
            self.registers.write_gp(rd, result)
        } else {
            false
        }
    }
}