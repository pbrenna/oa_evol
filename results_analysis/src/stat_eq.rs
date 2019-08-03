extern crate clap;
extern crate oarray;

use clap::{App, Arg};

use oarray::OArray;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let matches = App::new("StatEq")
        .version("0.1.0")
        .author("Pietro Brenna <p.brenna2@campus.unimib.Iterator>")
        .about("Checks statistical equivalence. Exit status 0: equivalent; 1: not equivalent")
        .arg(
            Arg::with_name("file1")
                .takes_value(true)
                .help("First OA file")
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .takes_value(true)
                .help("Second OA file")
                .required(true),
        )
        .get_matches();

    let f1 = matches.value_of_os("file1").unwrap();
    let f2 = matches.value_of_os("file2").unwrap();
    let f1content = read_to_string(f1).unwrap();
    let f2content = read_to_string(f2).unwrap();
    let mut oa1 = OArray::from(f1content.as_str());
    oa1.sort_rows(None);
    let mut oa2 = OArray::from(f2content.as_str());
    oa2.sort_rows(None);
    let eq = if oa1.ngrande != oa2.ngrande || oa1.k != oa2.k {
        false
    } else {
        oa1.d == oa2.d
    };
    if eq {
        println!("Statisticamente equivalenti");
        exit(0);
    } else {
        println!("Non statisticamente equivalenti");
        exit(1);
    }
}
