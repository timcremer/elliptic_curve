mod curve;
mod field;
use crate::field::FieldElement;
use crate::curve::{Curve, Point};
use num_bigint::BigInt;

fn main() {
    let curve = Curve::new(
        FieldElement::new(2u32.into(), 97u32.into()),
        FieldElement::new(3u32.into(), 97u32.into()),
    );

    let p = Point::new(
        FieldElement::new(3u32.into(), 97u32.into()),
        FieldElement::new(6u32.into(), 97u32.into()),
    );

    let q = Point::new(
        FieldElement::new(80u32.into(), 97u32.into()),
        FieldElement::new(10u32.into(), 97u32.into()),
    );

    let r = curve.double_point(&p);
    println!("R: {:?}", r);
}
