extern crate oarray;
use oarray::binary_strings;
use std::env;
use std::io;

fn main() {
    let len: usize = env::args().nth(1).unwrap().parse().unwrap();
    let weight: usize = env::args().nth(2).unwrap().parse().unwrap();
    let i = binary_strings::BinaryStringIterator::new(len);
    for v in i {
        if binary_strings::hamming_weight(&v) == weight {
            let v :Vec<u8> = v.iter().map(|&i| if i { 1u8} else {0}).collect();
            println!("{:?}", v);
        }
    }
}
