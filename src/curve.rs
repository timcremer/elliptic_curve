use crate::field::FieldElement;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

#[derive(Clone, Debug)]
pub struct Curve {
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Curve {
    pub fn new(a: FieldElement, b: FieldElement) -> Self {
        Curve { a, b }
    }



    pub fn add_points(&self, p1: &Point, p2: &Point) -> Point {
        if p1.x.is_none() {
            return p2.clone();
        }
        if p2.x.is_none() {
            return p1.clone();
        }
        if p1.x == p2.x && p1.y != p2.y {
            return Point::infinity();
        }
        if p1.x == p2.x && p1.y == p2.y {
            return self.double_point(p1);
        }
        
        let x1 = &p1.x.as_ref().unwrap().num.to_bigint().unwrap();
        let y1 = &p1.y.as_ref().unwrap().num.to_bigint().unwrap();
        let x2 = &p2.x.as_ref().unwrap().num.to_bigint().unwrap();
        let y2 = &p2.y.as_ref().unwrap().num.to_bigint().unwrap();
        let prime = &p1.x.as_ref().unwrap().prime.to_bigint().unwrap();

        let m: BigInt = ((y2 - y1) % prime) * modinv(&(x2 - x1), prime).unwrap() % prime;
        let x3 = &(((m.modpow(&BigInt::from(2u8), prime) - x1 - x2) % prime) + prime % prime);
        let y3 = (m * (x1 - x3) - y1) % prime + prime % prime;
        Point::new(
            FieldElement::new(x3.to_biguint().unwrap(), prime.to_biguint().unwrap()),
            FieldElement::new(y3.to_biguint().unwrap(), prime.to_biguint().unwrap()),
        )

    }

    pub fn double_point(&self, p: &Point) -> Point {
        if p.x.is_none() {
            return Point::infinity();
        }
        let x = &p.x.as_ref().unwrap().num.to_bigint().unwrap();
        let y = &p.y.as_ref().unwrap().num.to_bigint().unwrap();
        let prime = &p.x.as_ref().unwrap().prime.to_bigint().unwrap();

        if y.is_zero() {
            return Point::infinity();
        }

        let m: BigInt = (BigInt::from(3u8) * x.modpow(&BigInt::from(2u8), prime)
        + (&self.a.num.to_bigint().unwrap() % prime) * modinv(&(BigInt::from(2u8) * y), prime).unwrap()) % prime;
        let xp: BigInt = (m.modpow(&BigInt::from(2u8), prime) - BigInt::from(2u8) * x) % prime + prime % prime;
        Point::new(
            FieldElement::new(xp.to_biguint().unwrap(), prime.to_biguint().unwrap()),
            FieldElement::new(((((m * (x - xp) - y) % prime) + prime) % prime).to_biguint().unwrap(), prime.to_biguint().unwrap()),
        )
    }


    pub fn is_on_curve(&self, point: &Point) -> bool {
        if point.x.is_none() && point.y.is_none() {
            return true; // Point at infinity is on the curve
        }
        if let (Some(x), Some(y)) = (&point.x, &point.y) {
            let left = y.num.modpow(&BigUint::from(2u8), &y.prime);
            let right = (x.num.modpow(&BigUint::from(3u8), &x.prime) + 
                         self.a.num.modpow(&BigUint::from(1u8), &x.prime) * x.num.modpow(&BigUint::from(1u8), &x.prime) + 
                         self.b.num.modpow(&BigUint::from(1u8), &x.prime)) % &x.prime;
            return left == right;
        }
        false
    }

    pub fn order(&self, p: &Point) -> BigUint {
        let mut count = BigUint::from(1u8);
        let mut current = p.clone();
        while current.x.is_some() {
            current = self.add_points(&current, p);
            count += 1u8;
        }
        count
    }

}



#[derive(Clone, Debug)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

impl Point {
    pub fn infinity() -> Self {
        Point { x: None, y: None }
    }

    pub fn new(x: FieldElement, y: FieldElement) -> Self {
        Point { x: Some(x), y: Some(y) }
    }
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _y) = extended_gcd(a.clone(), m.clone());
    if g != BigInt::one() {
        return None;
    }
    Some((x % m + m) % m)
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        return (a, BigInt::one(), BigInt::zero());
    }
    let mut old_r = a;
    let mut r = b;
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();
    let mut old_t = BigInt::zero();
    let mut t = BigInt::one();

    while !r.is_zero() {
        let q = &old_r / &r;
        let tmp_r = old_r - &q * &r;
        old_r = r;
        r = tmp_r;

        let tmp_s = old_s - &q * &s;
        old_s = s;
        s = tmp_s;

        let tmp_t = old_t - &q * &t;
        old_t = t;
        t = tmp_t;
    }
    (old_r, old_s, old_t)
}