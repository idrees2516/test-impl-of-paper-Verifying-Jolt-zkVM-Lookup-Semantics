use super::{PedersenCommitment, PoseidonHash, Polynomial};
use crate::field::Fr;

pub struct ProofSystem {
    commitment: PedersenCommitment,
    hasher: PoseidonHash,
}

impl ProofSystem {
    pub fn new() -> Self {
        ProofSystem {
            commitment: PedersenCommitment::new(32),
            hasher: PoseidonHash::new(),
        }
    }

    pub fn create_proof(&mut self, witness: &[Fr], public_inputs: &[Fr]) -> Proof {
        // Generate polynomials from witness and constraints
        let witness_poly = Polynomial::new(witness.to_vec());
        
        // Create commitments
        let witness_commitment = self.commitment.commit(witness);
        
        // Generate proof components
        let mut transcript = Vec::new();
        transcript.extend_from_slice(public_inputs);
        transcript.push(witness_commitment);
        
        let challenge = self.hasher.hash(&transcript);
        
        // Evaluate polynomials at challenge point
        let evaluation = witness_poly.evaluate(challenge);
        
        Proof {
            witness_commitment,
            evaluation,
            challenge,
        }
    }

    pub fn verify(&mut self, proof: &Proof, public_inputs: &[Fr]) -> bool {
        // Reconstruct transcript
        let mut transcript = Vec::new();
        transcript.extend_from_slice(public_inputs);
        transcript.push(proof.witness_commitment);
        
        let challenge = self.hasher.hash(&transcript);
        
        // Verify proof components
        challenge == proof.challenge
    }
}

pub struct Proof {
    witness_commitment: Fr,
    evaluation: Fr,
    challenge: Fr,
}