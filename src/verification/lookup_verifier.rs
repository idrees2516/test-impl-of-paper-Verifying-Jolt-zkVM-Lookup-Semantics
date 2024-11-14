use crate::field::Fr;
use crate::crypto::merkle::MerkleTree;
use std::collections::HashMap;

/// Represents a lookup table entry
#[derive(Debug, Clone)]
pub struct LookupEntry {
    key: Fr,
    value: Fr,
    proof: Vec<Fr>,
}

/// Main lookup verifier for instruction verification
pub struct LookupVerifier {
    // Instruction lookup tables
    instruction_tables: HashMap<InstructionType, LookupTable>,
    // Subtable verifiers for decomposed lookups
    subtable_verifiers: Vec<SubtableVerifier>,
    // Permutation argument checker
    permutation_checker: PermutationChecker,
    // Range verification
    range_verifier: RangeVerifier,
}

impl LookupVerifier {
    pub fn new() -> Self {
        Self {
            instruction_tables: HashMap::new(),
            subtable_verifiers: Vec::new(),
            permutation_checker: PermutationChecker::new(),
            range_verifier: RangeVerifier::new(),
        }
    }

    /// Verify a lookup proof for an instruction
    pub fn verify_instruction_lookup(
        &self,
        instruction: &Instruction,
        proof: &LookupProof
    ) -> Result<bool, VerificationError> {
        // 1. Verify instruction encoding matches table
        let encoding = self.verify_instruction_encoding(instruction)?;
        
        // 2. Verify subtable lookups
        for (i, subtable_proof) in proof.subtable_proofs.iter().enumerate() {
            self.subtable_verifiers[i].verify(subtable_proof, &encoding)?;
        }
        
        // 3. Verify permutation argument
        self.permutation_checker.verify(&proof.permutation_proof)?;
        
        // 4. Verify range constraints
        self.range_verifier.verify(&proof.range_proof)?;
        
        Ok(true)
    }

    /// Verify instruction encoding matches lookup table
    fn verify_instruction_encoding(
        &self,
        instruction: &Instruction
    ) -> Result<InstructionEncoding, VerificationError> {
        let table = self.instruction_tables.get(&instruction.type_id())
            .ok_or(VerificationError::UnknownInstruction)?;
            
        table.verify_encoding(instruction)
    }
}

/// Verifies range constraints on values
struct RangeVerifier {
    bit_decomposition_checker: BitDecompositionChecker,
    range_proof_verifier: RangeProofVerifier,
}

impl RangeVerifier {
    fn new() -> Self {
        Self {
            bit_decomposition_checker: BitDecompositionChecker::new(),
            range_proof_verifier: RangeProofVerifier::new(),
        }
    }

    fn verify(&self, proof: &RangeProof) -> Result<bool, VerificationError> {
        // 1. Verify bit decomposition
        self.bit_decomposition_checker.verify(&proof.bit_proofs)?;
        
        // 2. Verify range constraints
        self.range_proof_verifier.verify(
            &proof.commitments,
            &proof.challenges,
            &proof.responses
        )?;
        
        Ok(true)
    }
} 