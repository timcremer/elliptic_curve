use crate::curve::Point;
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct PrivateKey(pub BigUint);

impl PrivateKey {
    pub fn from_biguint(v: BigUint) -> Self { PrivateKey(v) }
}

#[derive(Clone, Debug)]
pub struct PublicKey(pub Point);

impl PublicKey {
    pub fn generator() -> Self { PublicKey(Point::infinity()) }
}
