use crate::field::Fr;
use rayon::prelude::*;
use std::sync::atomic::{fence, Ordering};

/// SIMD Vector Processing Unit with constant-time operations
pub struct VectorUnit {
    // Vector registers
    registers: Vec<VectorRegister>,
    
    // Mask registers for predicated execution
    mask_registers: Vec<VectorMask>,
    
    // Configuration
    vector_length: usize,
    max_elements: usize,
}

#[derive(Clone)]
pub struct VectorRegister {
    elements: Vec<Fr>,
    flags: VectorFlags,
}

#[derive(Clone)]
pub struct VectorMask {
    mask: Vec<bool>,
    predicate_stack: Vec<Vec<bool>>,
}

#[derive(Clone, Copy)]
pub struct VectorFlags {
    overflow: bool,
    underflow: bool,
    divide_by_zero: bool,
    invalid_op: bool,
}

impl VectorUnit {
    pub fn new(vector_length: usize, num_registers: usize) -> Self {
        Self {
            registers: vec![VectorRegister::new(vector_length); num_registers],
            mask_registers: vec![VectorMask::new(vector_length); 8],
            vector_length,
            max_elements: vector_length,
        }
    }

    /// Vector arithmetic operations
    pub fn vector_add(&mut self, vd: usize, vs1: usize, vs2: usize, mask: usize) 
        -> Result<(), VectorError> 
    {
        self.check_register_indices(&[vd, vs1, vs2])?;
        let mask_reg = &self.mask_registers[mask];
        
        // Constant-time vector addition
        fence(Ordering::SeqCst);
        
        let vs1_reg = &self.registers[vs1];
        let vs2_reg = &self.registers[vs2];
        let vd_reg = &mut self.registers[vd];
        
        vd_reg.elements.par_iter_mut()
            .zip(vs1_reg.elements.par_iter())
            .zip(vs2_reg.elements.par_iter())
            .zip(mask_reg.mask.par_iter())
            .for_each(|(((vd_elem, &vs1_elem), &vs2_elem), &mask)| {
                if mask {
                    *vd_elem = vs1_elem + vs2_elem;
                }
            });
            
        Ok(())
    }

    /// Vector multiplication with overflow detection
    pub fn vector_multiply(&mut self, vd: usize, vs1: usize, vs2: usize, mask: usize)
        -> Result<(), VectorError>
    {
        self.check_register_indices(&[vd, vs1, vs2])?;
        let mask_reg = &self.mask_registers[mask];
        
        fence(Ordering::SeqCst);
        
        let vs1_reg = &self.registers[vs1];
        let vs2_reg = &self.registers[vs2];
        let vd_reg = &mut self.registers[vd];
        
        let mut overflow = false;
        
        vd_reg.elements.par_iter_mut()
            .zip(vs1_reg.elements.par_iter())
            .zip(vs2_reg.elements.par_iter())
            .zip(mask_reg.mask.par_iter())
            .for_each(|(((vd_elem, &vs1_elem), &vs2_elem), &mask)| {
                if mask {
                    let (result, did_overflow) = vs1_elem.overflowing_mul(&vs2_elem);
                    *vd_elem = result;
                    overflow |= did_overflow;
                }
            });
            
        vd_reg.flags.overflow = overflow;
        Ok(())
    }

    /// Vector division with safety checks
    pub fn vector_divide(&mut self, vd: usize, vs1: usize, vs2: usize, mask: usize)
        -> Result<(), VectorError>
    {
        self.check_register_indices(&[vd, vs1, vs2])?;
        let mask_reg = &self.mask_registers[mask];
        
        fence(Ordering::SeqCst);
        
        let vs1_reg = &self.registers[vs1];
        let vs2_reg = &self.registers[vs2];
        let vd_reg = &mut self.registers[vd];
        
        let mut divide_by_zero = false;
        
        vd_reg.elements.par_iter_mut()
            .zip(vs1_reg.elements.par_iter())
            .zip(vs2_reg.elements.par_iter())
            .zip(mask_reg.mask.par_iter())
            .for_each(|(((vd_elem, &vs1_elem), &vs2_elem), &mask)| {
                if mask {
                    if vs2_elem.is_zero() {
                        divide_by_zero = true;
                    } else {
                        *vd_elem = vs1_elem / vs2_elem;
                    }
                }
            });
            
        if divide_by_zero {
            vd_reg.flags.divide_by_zero = true;
            return Err(VectorError::DivideByZero);
        }
        
        Ok(())
    }

    /// Vector reduction operations
    pub fn vector_reduce(&self, vs: usize, op: ReduceOp, mask: usize) 
        -> Result<Fr, VectorError>
    {
        self.check_register_indices(&[vs])?;
        let mask_reg = &self.mask_registers[mask];
        let vs_reg = &self.registers[vs];
        
        let result = match op {
            ReduceOp::Sum => vs_reg.elements.par_iter()
                .zip(mask_reg.mask.par_iter())
                .filter(|(_, &mask)| mask)
                .map(|(&elem, _)| elem)
                .reduce(|| Fr::zero(), |a, b| a + b),
                
            ReduceOp::Product => vs_reg.elements.par_iter()
                .zip(mask_reg.mask.par_iter())
                .filter(|(_, &mask)| mask)
                .map(|(&elem, _)| elem)
                .reduce(|| Fr::one(), |a, b| a * b),
                
            ReduceOp::Max => vs_reg.elements.par_iter()
                .zip(mask_reg.mask.par_iter())
                .filter(|(_, &mask)| mask)
                .map(|(&elem, _)| elem)
                .reduce(|| Fr::min_value(), |a, b| Fr::max(a, b)),
                
            ReduceOp::Min => vs_reg.elements.par_iter()
                .zip(mask_reg.mask.par_iter())
                .filter(|(_, &mask)| mask)
                .map(|(&elem, _)| elem)
                .reduce(|| Fr::max_value(), |a, b| Fr::min(a, b)),
        };
        
        Ok(result)
    }

    /// Vector permutation with bounds checking
    pub fn vector_permute(&mut self, vd: usize, vs: usize, indices: &[usize], mask: usize)
        -> Result<(), VectorError>
    {
        self.check_register_indices(&[vd, vs])?;
        let mask_reg = &self.mask_registers[mask];
        
        let vs_reg = &self.registers[vs];
        let vd_reg = &mut self.registers[vd];
        
        vd_reg.elements.par_iter_mut()
            .zip(indices.par_iter())
            .zip(mask_reg.mask.par_iter())
            .try_for_each(|((vd_elem, &idx), &mask)| {
                if mask {
                    if idx >= self.vector_length {
                        return Err(VectorError::IndexOutOfBounds);
                    }
                    *vd_elem = vs_reg.elements[idx];
                }
                Ok(())
            })?;
            
        Ok(())
    }

    /// Vector comparison operations
    pub fn vector_compare(&mut self, vd: usize, vs1: usize, vs2: usize, op: CompareOp, mask: usize)
        -> Result<(), VectorError>
    {
        self.check_register_indices(&[vd, vs1, vs2])?;
        let mask_reg = &self.mask_registers[mask];
        
        let vs1_reg = &self.registers[vs1];
        let vs2_reg = &self.registers[vs2];
        let vd_reg = &mut self.registers[vd];
        
        vd_reg.elements.par_iter_mut()
            .zip(vs1_reg.elements.par_iter())
            .zip(vs2_reg.elements.par_iter())
            .zip(mask_reg.mask.par_iter())
            .for_each(|(((vd_elem, &vs1_elem), &vs2_elem), &mask)| {
                if mask {
                    *vd_elem = match op {
                        CompareOp::Eq => Fr::from((vs1_elem == vs2_elem) as u64),
                        CompareOp::Ne => Fr::from((vs1_elem != vs2_elem) as u64),
                        CompareOp::Lt => Fr::from((vs1_elem < vs2_elem) as u64),
                        CompareOp::Le => Fr::from((vs1_elem <= vs2_elem) as u64),
                        CompareOp::Gt => Fr::from((vs1_elem > vs2_elem) as u64),
                        CompareOp::Ge => Fr::from((vs1_elem >= vs2_elem) as u64),
                    };
                }
            });
            
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum ReduceOp {
    Sum,
    Product,
    Max,
    Min,
}

#[derive(Clone, Copy)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug)]
pub enum VectorError {
    InvalidRegister,
    IndexOutOfBounds,
    DivideByZero,
    InvalidOperation,
}
 