extern crate oarray;

use oarray::OArray;
use std::io::{self, Read};

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let oa = OArray::from(buffer.as_str());
    let linear = oa.check_linear();
    if linear {
        println!("Linear");
    } else {
        println!("Not linear");
    }
    Ok(())
}