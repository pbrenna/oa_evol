extern crate oarray;

use oarray::wtform::*;
use oarray::FitnessFunction;
use oarray::OArray;
use std::env;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let t: u32 = env::args()
        .nth(1)
        .expect("Fornire parametro t come argomento")
        .parse()
        .unwrap();
    io::stdin().read_to_string(&mut buffer)?;
    let mut oa = OArray::from(buffer.as_str());
    let truth = oa.truth_table();
    let tform = PolarTruthTable::from(&truth).walsh_tform();
    oa.target_t = t;
    println!("Target t: {}", t);
    println!("Array:\n{}", oa);
    oa.fitness_f = FitnessFunction::Delta;
    println!("Fitness(Delta): {}", oa.fitness());
    oa.fitness_f = FitnessFunction::Walsh(2);
    println!("Fitness(Walsh(2)): {}", oa.fitness());
    for i in 1..6 {
        println!("cidev({}): {}", i, tform.cidev(i));
    }
    println!("radius: {}", tform.radius());
    println!("Truth table:\n{}", truth);
    println!("Walsh transform:\n{}", tform);
    Ok(())
}
