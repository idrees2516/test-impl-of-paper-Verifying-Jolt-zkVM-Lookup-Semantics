use crate::field::Fr;

#[derive(Clone)]
pub struct Polynomial {
    coefficients: Vec<Fr>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<Fr>) -> Self {
        let mut poly = Polynomial { coefficients };
        poly.normalize();
        poly
    }

    pub fn evaluate(&self, point: Fr) -> Fr {
        let mut result = Fr::zero();
        let mut power = Fr::one();
        
        for coefficient in &self.coefficients {
            result += *coefficient * power;
            power *= point;
        }
        
        result
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let n = self.coefficients.len() + other.coefficients.len() - 1;
        let mut result = vec![Fr::zero(); n];
        
        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                result[i + j] += *a * *b;
            }
        }
        
        Polynomial::new(result)
    }

    pub fn divide(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        let mut quotient = vec![Fr::zero(); self.coefficients.len()];
        let mut remainder = self.coefficients.clone();
        
        let divisor_deg = divisor.degree();
        let self_deg = self.degree();
        
        for i in (0..=self_deg - divisor_deg).rev() {
            let factor = remainder[i + divisor_deg] / divisor.coefficients[divisor_deg];
            quotient[i] = factor;
            
            for j in 0..=divisor_deg {
                remainder[i + j] -= factor * divisor.coefficients[j];
            }
        }
        
        (
            Polynomial::new(quotient),
            Polynomial::new(remainder[0..divisor_deg].to_vec())
        )
    }

    fn normalize(&mut self) {
        while self.coefficients.last().map_or(false, |c| c.is_zero()) {
            self.coefficients.pop();
        }
        if self.coefficients.is_empty() {
            self.coefficients.push(Fr::zero());
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}