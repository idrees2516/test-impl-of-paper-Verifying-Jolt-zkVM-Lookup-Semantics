use super::hash::PoseidonHash;
use crate::field::Fr;
use rand::RngCore;

pub struct PedersenCommitment {
    generators: Vec<Fr>,
    blinding_factor: Fr,
}

impl PedersenCommitment {
    pub fn new(num_generators: usize) -> Self {
        let mut rng = rand::thread_rng();
        let generators = (0..num_generators)
            .map(|_| Fr::random(&mut rng))
            .collect();
        
        PedersenCommitment {
            generators,
            blinding_factor: Fr::random(&mut rng),
        }
    }

    pub fn commit(&self, values: &[Fr]) -> Fr {
        assert_eq!(values.len(), self.generators.len());
        
        let mut commitment = Fr::zero();
        for (value, generator) in values.iter().zip(self.generators.iter()) {
            commitment += *value * generator;
        }
        commitment += self.blinding_factor
    }
}