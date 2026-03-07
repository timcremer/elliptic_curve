use num_bigint::BigUint;
use num_traits::Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        let num = if prime.is_zero() { num } else { num % &prime };
        FieldElement { num, prime }
    }
}
