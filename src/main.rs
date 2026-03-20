mod curve;
use crate::curve::{Curve, Point};
use num_bigint::{BigUint,BigInt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curve = Curve::new(
        2u32.into(),
        3u32.into(),
        97u32.into()
    )?;

    let p1 = Point::new(3u32.into(), 6u32.into());
    let p2 = Point::new(80u32.into(), 10u32.into());
    let p3 = curve.add_points(&p1, &p2)?;
    
    println!("P1 + P2 = {:?}", p3);
    
    let mut p4: Point = Point::infinity();
    for i in 0..5 {
        p4 = curve.add_points(&p4, &p3)?;
        println!(":({} * P3 = {:?}", i + 1, p4);
    }
    
    let p5 = curve.scalar_mult(&p3, &BigUint::from(5u32))?;
    println!("5 * P3 = {:?}", p5);
    //println!("11 * P3 = {:?}", p4);
    Ok(())
}
