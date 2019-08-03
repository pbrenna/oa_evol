extern crate oarray;

//use simplelog::*;
use std::env;

mod hill_climb;
use oarray::OArray;
use std::io::{self};

fn main() {
    let t = env::args()
        .nth(1)
        .expect("Fornire parametro t come argomento")
        .parse()
        .unwrap();
    'outer: loop {
        let mut buffer = String::new();
        while let Ok(num) = io::stdin().read_line(&mut buffer) {
            if num == 1 {
                break;
            }
        }
        let mut oa = OArray::from(buffer.trim());
        oa.fitness_f = oarray::FitnessFunction::WalshFaster(2);
        oa.target_t = t;
        let old_fit = oa.fitness();
        let old_oa = oa.clone();
        let mut count = 0;
        loop {
            let out = hill_climb::hill_climb(oa);
            if out.0.fitness() > -1.0{
                println!("{}", old_oa);
                println!("{}", out.0);
                println!("{}, {}, {}", old_fit, out.0.fitness(), count);
                break 'outer;
            }
            if !out.1 {
                println!("nisba ({})",count);
                break;
            }
            oa = out.0;
            count += 1;
        }
    }
}
