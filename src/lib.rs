pub mod field;
pub mod curve;
pub mod key;

pub use field::FieldElement;
pub use curve::{Curve, Point};
pub use key::{PrivateKey, PublicKey};

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::One;

    #[test]
    fn basic_types_compile() {
        let p = BigUint::one();
        let fe = FieldElement::new(p.clone(), BigUint::from(7u8));
        let _curve = Curve::new(fe.clone(), fe);
        let _sk = PrivateKey::from_biguint(p);
        let _pk = PublicKey::generator();
        assert!(matches!(_pk.0.x, None));
    }
}
