#[cfg(test)]
mod tests {
    use jolt_zkvm::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut ctx = ExecutionContext::new(64);
        
        // Test ADD
        ctx.registers.write_gp(1, 5);
        ctx.registers.write_gp(2, 3);
        let add = instructions::AddInstruction;
        assert!(ctx.execute_instruction(&add, 1, 2, 3, 64));
        assert_eq!(ctx.registers.read_gp(3), Some(8));
        
        // Add more instruction tests
    }
}