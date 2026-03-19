mod curve;
mod field;
use crate::field::FieldElement;
use crate::curve::{Curve, Point};
use num_bigint::BigInt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curve = Curve::new(
        2u32.into(),
        3u32.into(),
        97u32.into()
    )?;

    let p1 = Point::new(3u32.into(), 6u32.into());
    let p2 = Point::new(80u32.into(), 10u32.into());
    let p3 = curve.add_points(&p1, &p2);
    println!("P1 + P2 = {:?}", p3);

  
    
    Ok(())
}
