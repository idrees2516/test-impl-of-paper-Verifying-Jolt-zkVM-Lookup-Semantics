use cranelift::prelude::*;
use cranelift_module::{Module, Linkage};

pub struct JitCompiler {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module,
}

impl JitCompiler {
    pub fn new() -> Self {
        let builder = settings::builder();
        let flags = settings::Flags::new(builder);
        let isa = isa::lookup(target_lexicon::HOST).unwrap();
        
        JitCompiler {
            builder_context: FunctionBuilderContext::new(),
            ctx: codegen::Context::new(),
            module: Module::new(isa),
        }
    }

    pub fn compile_function(&mut self, instructions: &[Instruction]) -> Result<*const u8, JitError> {
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        
        // Create entry block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        
        // Compile instructions
        for inst in instructions {
            self.compile_instruction(&mut builder, inst)?;
        }
        
        // Finalize function
        builder.seal_all_blocks();
        let id = self.module.declare_function(
            "jit_func",
            Linkage::Export,
            &self.ctx.func.signature,
        )?;
        
        self.module.define_function(id, &mut self.ctx)?;
        self.module.clear_context(&mut self.ctx);
        
        Ok(self.module.get_finalized_function(id))
    }
} 