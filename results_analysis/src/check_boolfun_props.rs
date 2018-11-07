extern crate oarray;

use oarray::wtform::*;
use oarray::OArray;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let oa = OArray::from(buffer.as_str());
    let truth = oa.truth_table();
    let tform = PolarTruthTable::from(&truth).walsh_tform();
    println!("Truth table");
    println!("{}", truth);
    println!("Walsh transform");
    println!("{}", tform);
    for i in 1..6 {
        println!("cidev({}): {}", i, tform.cidev(i));
    }
    println!("radius: {}", tform.radius());
    Ok(())
}
