use crate::field::Fr;
use crate::polynomial::*;
use std::collections::{BTreeMap, HashMap};
use rayon::prelude::*;

pub struct LookupTable {
    entries: BTreeMap<Fr, TableEntry>,
    preprocessed_data: PreprocessedData,
    multiset_checks: Vec<MultisetCheck>,
    compression_scheme: TableCompression,
}

struct TableEntry {
    value: Fr,
    metadata: EntryMetadata,
    auxiliary_data: Vec<Fr>,
}

struct EntryMetadata {
    index: usize,
    hash: Fr,
    merkle_path: Vec<Fr>,
    commitment: Fr,
}

struct PreprocessedData {
    polynomial_evaluations: Vec<Fr>,
    barycentric_weights: Vec<Fr>,
    lagrange_coefficients: Vec<Fr>,
    vanishing_polynomial: Polynomial,
}

impl LookupTable {
    pub fn new(size: usize, width: usize) -> Self {
        let preprocessed = Self::preprocess_table(size, width);
        let compression = TableCompression::new(width);
        
        LookupTable {
            entries: BTreeMap::new(),
            preprocessed_data: preprocessed,
            multiset_checks: Vec::new(),
            compression_scheme: compression,
        }
    }

    fn preprocess_table(size: usize, width: usize) -> PreprocessedData {
        let domain = EvaluationDomain::new(size);
        let vanishing = domain.vanishing_polynomial();
        
        // Compute barycentric weights
        let weights = domain.points().par_iter().map(|&x| {
            let mut w = Fr::one();
            for &y in domain.points() {
                if x != y {
                    w *= (x - y);
                }
            }
            w.inverse().unwrap()
        }).collect();

        // Precompute Lagrange coefficients
        let lagrange = (0..size).into_par_iter().map(|i| {
            let mut coeffs = vec![Fr::zero(); size];
            coeffs[i] = Fr::one();
            Polynomial::from_coefficients_vec(coeffs)
        }).collect();

        PreprocessedData {
            polynomial_evaluations: vec![Fr::zero(); size],
            barycentric_weights: weights,
            lagrange_coefficients: lagrange,
            vanishing_polynomial: vanishing,
        }
    }

    pub fn insert(&mut self, key: Fr, value: Fr, auxiliary: Vec<Fr>) {
        let index = self.entries.len();
        let hash = self.compression_scheme.hash(&[key, value]);
        
        let merkle_path = self.build_merkle_path(index, &hash);
        let commitment = self.commit_entry(&key, &value, &auxiliary);
        
        let entry = TableEntry {
            value,
            metadata: EntryMetadata {
                index,
                hash,
                merkle_path,
                commitment,
            },
            auxiliary_data: auxiliary,
        };
        
        self.entries.insert(key, entry);
        self.update_preprocessed_data(key, value);
    }

    fn update_preprocessed_data(&mut self, key: Fr, value: Fr) {
        let index = self.entries.len() - 1;
        let domain = EvaluationDomain::new(self.entries.len());
        
        // Update polynomial evaluations
        for (i, &point) in domain.points().iter().enumerate() {
            let contribution = value * self.preprocessed_data.barycentric_weights[index] 
                           / (point - key);
            self.preprocessed_data.polynomial_evaluations[i] += contribution;
        }
        
        // Update multiset checks
        self.update_multiset_checks(key, value);
    }

    fn update_multiset_checks(&mut self, key: Fr, value: Fr) {
        let mut new_checks = Vec::new();
        
        // Frequency check
        let freq_poly = self.build_frequency_polynomial(&key, &value);
        new_checks.push(MultisetCheck::Frequency(freq_poly));
        
        // Permutation check
        let perm_poly = self.build_permutation_polynomial(&key, &value);
        new_checks.push(MultisetCheck::Permutation(perm_poly));
        
        self.multiset_checks.extend(new_checks);
    }
}

struct TableCompression {
    width: usize,
    poseidon: PoseidonHasher,
    compression_matrix: Vec<Vec<Fr>>,
}

impl TableCompression {
    fn new(width: usize) -> Self {
        let matrix = Self::generate_compression_matrix(width);
        TableCompression {
            width,
            poseidon: PoseidonHasher::new(),
            compression_matrix: matrix,
        }
    }

    fn generate_compression_matrix(width: usize) -> Vec<Vec<Fr>> {
        let mut matrix = vec![vec![Fr::zero(); width]; width];
        let mut rng = rand::thread_rng();
        
        // Generate random invertible matrix
        loop {
            for i in 0..width {
                for j in 0..width {
                    matrix[i][j] = Fr::random(&mut rng);
                }
            }
            if Self::is_invertible(&matrix) {
                break;
            }
        }
        matrix
    }

    fn compress(&self, input: &[Fr]) -> Fr {
        let mut result = Fr::zero();
        for (row, &value) in self.compression_matrix.iter().zip(input) {
            let mut term = Fr::zero();
            for (&coeff, &base) in row.iter().zip(input) {
                term += coeff * base;
            }
            result += term * value;
        }
        self.poseidon.hash(&[result])
    }
}

enum MultisetCheck {
    Frequency(Polynomial),
    Permutation(Polynomial),
    Custom(Box<dyn Fn(&[Fr]) -> Polynomial>),
} 