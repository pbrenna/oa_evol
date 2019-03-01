extern crate oarray;

use oarray::OArray;
use std::io::{self, Read};

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut oa = OArray::from(buffer.as_str());
    oa.target_t = 3;
    let linear = oa.check_linear();
    if linear {
        println!("Linear");
    } else {
        println!("Not linear");
    }
    use oarray::FitnessFunction::*;
    oa.fitness_f = WalshRec(2);
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = DeltaFast;
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = WalshFaster(2);
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = Walsh(2);
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = Delta;
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = Cidev;
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    oa.fitness_f = SheerLuck;
    println!("{:?}: {}", oa.fitness_f, oa.fitness());
    Ok(())
}