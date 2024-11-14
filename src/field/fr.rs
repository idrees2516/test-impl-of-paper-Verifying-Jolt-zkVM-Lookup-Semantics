use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use rand::RngCore;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fr(u64);

impl Fr {
    pub const MODULUS: u64 = 0xFFFFFFFF00000001;

    pub fn zero() -> Self {
        Fr(0)
    }

    pub fn one() -> Self {
        Fr(1)
    }

    pub fn random(rng: &mut impl RngCore) -> Self {
        Fr(rng.next_u64() % Self::MODULUS)
    }

    pub fn pow(&self, exp: u64) -> Self {
        let mut base = *self;
        let mut result = Fr::one();
        let mut exp = exp;
        
        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base *= base;
            exp >>= 1;
        }
        
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(self.pow(Self::MODULUS - 2))
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for Fr {
    fn from(value: u64) -> Self {
        Fr(value % Self::MODULUS)
    }
}

impl Add for Fr {
    type Output = Fr;
    
    fn add(self, rhs: Fr) -> Fr {
        Fr((self.0 + rhs.0) % Self::MODULUS)
    }
}

impl AddAssign for Fr {
    fn add_assign(&mut self, rhs: Fr) {
        *self = *self + rhs;
    }
}

impl Sub for Fr {
    type Output = Fr;
    
    fn sub(self, rhs: Fr) -> Fr {
        if self.0 >= rhs.0 {
            Fr(self.0 - rhs.0)
        } else {
            Fr(Self::MODULUS - (rhs.0 - self.0))
        }
    }
}

impl SubAssign for Fr {
    fn sub_assign(&mut self, rhs: Fr) {
        *self = *self - rhs;
    }
}

impl Mul for Fr {
    type Output = Fr;
    
    fn mul(self, rhs: Fr) -> Fr {
        Fr(((self.0 as u128 * rhs.0 as u128) % Self::MODULUS as u128) as u64)
    }
}

impl MulAssign for Fr {
    fn mul_assign(&mut self, rhs: Fr) {
        *self = *self * rhs;
    }
}