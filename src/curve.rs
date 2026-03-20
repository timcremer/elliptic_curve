use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Curve {
    pub a: BigUint,
    pub b: BigUint,
    pub prime: BigUint,
}

#[derive(Debug, Error)]
pub enum CurveError {
    #[error("Singular Curve: Discriminant is 0")]
    SingularCurve,
    #[error("Point is not on the curve")]
    PointNotOnCurve,
}


impl Curve {
    pub fn new(a: BigUint, b: BigUint, prime: BigUint) -> Result<Self, CurveError> {
        let discriminant = BigUint::from(4u8) * a.modpow(&BigUint::from(3u8), &prime) + 
                           BigUint::from(27u8) * b.modpow(&BigUint::from(2u8), &prime) % &prime;
        if discriminant.is_zero() {
            return Err(CurveError::SingularCurve);
        }
        Ok(Curve { a, b, prime })
    }



    pub fn add_points(&self, p1: &Point, p2: &Point) -> Result<Point, CurveError> {
        if !self.is_on_curve(p1) {
            println!("Point {:?} is not on the curve, add points", p1);
            return Err(CurveError::PointNotOnCurve);
        }
        if !self.is_on_curve(p2) {
            println!("Point {:?} is not on the curve, add points", p2);
            return Err(CurveError::PointNotOnCurve);
        }
        if p1.x.is_none() {
            return Ok(p2.clone());
        }
        if p2.x.is_none() {
            return Ok(p1.clone());
        }
        if p1.x == p2.x && p1.y != p2.y {
            return Ok(Point::infinity());
        }
        if p1==p2 {
            return self.double_point(p1);
        }
        
        
        let x1 = &p1.x.as_ref().unwrap().to_bigint().unwrap();
        let y1 = &p1.y.as_ref().unwrap().to_bigint().unwrap();
        let x2 = &p2.x.as_ref().unwrap().to_bigint().unwrap();
        let y2 = &p2.y.as_ref().unwrap().to_bigint().unwrap();
        let prime = &self.prime.to_bigint().unwrap();
        let m: BigInt = ((y2 - y1) % prime ) * modinv(&(x2 - x1), prime).unwrap() % prime;
        let x3 = &((((m.modpow(&BigInt::from(2u8), prime) - x1 - x2) % prime) + prime) % prime);
        let y3 = ((m * (x1 - x3) - y1) % prime + prime) % prime;
        Ok(Point::new_mod(
            x3.to_biguint().unwrap(),
            y3.to_biguint().unwrap(),
            self.prime.clone()
        ))

    }

    pub fn double_point(&self, p: &Point) -> Result<Point, CurveError> {
        if !self.is_on_curve(p) {
            println!("Point {:?} is not on the curve, double point", p);
            return Err(CurveError::PointNotOnCurve);
        }
        if p.x.is_none() {
            return Ok(Point::infinity());
        }
        let x = &p.x.as_ref().unwrap().to_bigint().unwrap();
        let y = &p.y.as_ref().unwrap().to_bigint().unwrap();
        let prime = &self.prime.to_bigint().unwrap();
        
        if y.is_zero() {
            return Ok(Point::infinity());
        }

        let m: BigInt = ((BigInt::from(3u8) * x.modpow(&BigInt::from(2u8), prime)
        + (&self.a.to_bigint().unwrap() % prime)) * modinv(&(BigInt::from(2u8) * y), prime).unwrap()) % prime;
        let xp: BigInt = ((m.modpow(&BigInt::from(2u8), prime) - BigInt::from(2u8) * x) % prime + prime) % prime;
        Ok(Point::new_mod(
            xp.to_biguint().unwrap(),
            (((m * (x - xp) - y) % prime + prime) % prime).to_biguint().unwrap(),
            self.prime.clone()
        ))
    }

    pub fn scalar_mult(&self, p: &Point, n: &BigUint) -> Result<Point, CurveError> {
        if !self.is_on_curve(p) {
            println!("Point {:?} is not on the curve, scalar mult", p);
            return Err(CurveError::PointNotOnCurve);
        }
        let mut result = Point::infinity();
        let mut pot_2 = p.clone();
        for i in 0..(n.bits()) {
            if n.bit(i) {
                result = self.add_points(&result, &pot_2)?;
            }
            pot_2 = self.double_point(&pot_2)?;
        }
        Ok(result)

    }

    pub fn is_on_curve(&self, point: &Point) -> bool {
        if point.x.is_none() && point.y.is_none() {
            return true; // Point at infinity is on the curve
        }
        if let (Some(x), Some(y)) = (&point.x, &point.y) {
            let left = y.modpow(&BigUint::from(2u8), &self.prime);
            let right = (x.modpow(&BigUint::from(3u8), &self.prime) + 
                         self.a.modpow(&BigUint::from(1u8), &self.prime) * x.modpow(&BigUint::from(1u8), &self.prime) + 
                         self.b.modpow(&BigUint::from(1u8), &self.prime)) % &self.prime;
            return left == right;
        }
        false
    }

    pub fn order(&self, p: &Point) -> Result<BigUint, CurveError> {
        let mut count = BigUint::from(1u8);
        let mut current = p.clone();
        while current.x.is_some() {
            current = self.add_points(&current, p)?;
            count += 1u8;
        }
        Ok(count)
    }

}



#[derive(Clone, Debug,Eq, PartialEq)]
pub struct Point {
    pub x: Option<BigUint>,
    pub y: Option<BigUint>,
}

impl Point {
    pub fn infinity() -> Self {
        Point { x: None, y: None }
    }

    pub fn new(x: BigUint, y: BigUint) -> Self {
        Point { x: Some(x), y: Some(y) }
    }

    pub fn new_mod(x: BigUint, y: BigUint, prime: BigUint) -> Self {
        Point { 
            x: Some(x % &prime), 
            y: Some(y % &prime) 
        }
    }
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _y) = extended_gcd(a.clone(), m.clone());
    if g != BigInt::one() && g != BigInt::from(-1) {
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