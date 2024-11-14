use super::hash::PoseidonHash;
use crate::field::Fr;

pub struct MerkleTree {
    levels: Vec<Vec<Fr>>,
    hasher: PoseidonHash,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Fr>) -> Self {
        let mut hasher = PoseidonHash::new();
        let mut levels = vec![leaves];
        
        while levels.last().unwrap().len() > 1 {
            let current_level = levels.last().unwrap();
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left = chunk[0];
                let right = chunk.get(1).copied().unwrap_or(Fr::zero());
                next_level.push(hasher.hash(&[left, right]));
            }
            
            levels.push(next_level);
        }
        
        MerkleTree { levels, hasher }
    }

    pub fn root(&self) -> Fr {
        *self.levels.last().unwrap().first().unwrap()
    }

    pub fn generate_proof(&self, index: usize) -> MerkleProof {
        let mut proof = Vec::new();
        let mut current_index = index;
        
        for level in 0..self.levels.len()-1 {
            let sibling_index = current_index ^ 1;
            if sibling_index < self.levels[level].len() {
                proof.push(self.levels[level][sibling_index]);
            }
            current_index /= 2;
        }
        
        MerkleProof { proof }
    }
}

pub struct MerkleProof {
    proof: Vec<Fr>,
}

impl MerkleProof {
    pub fn verify(&self, root: Fr, leaf: Fr, index: usize, hasher: &mut PoseidonHash) -> bool {
        let mut current = leaf;
        let mut current_index = index;
        
        for sibling in &self.proof {
            let (left, right) = if current_index % 2 == 0 {
                (current, *sibling)
            } else {
                (*sibling, current)
            };
            current = hasher.hash(&[left, right]);
            current_index /= 2;
        }
        
        current == root
    }
}