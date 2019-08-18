extern crate clap;
extern crate oarray;

use clap::{App, Arg};

use oarray::OArray;
use std::fs::read_to_string;
use std::io::{self, Read};

fn main() ->  io::Result<()> {
    let matches = App::new("w_distr")
        .version("0.1.0")
        .author("Pietro Brenna <p.brenna2@campus.unimib.it>")
        .about("Calculate weight distributions")
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("OA file. Use `-` for standard input")
                .required(true),
        )
        .get_matches();

    let f = matches.value_of_os("file").unwrap();
    let fcontent = if f == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        read_to_string(f)?
    };
    let oa = OArray::from(fcontent.as_str());
    println!("Zero: {:?}
Proper: {:?}", oa.zero_weight_d(), oa.proper_weight_d());
    Ok(())
}
