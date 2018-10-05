extern crate oarray;
extern crate pbr;
extern crate rand;
extern crate clap;
extern crate spiril;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::*;
use std::fs::File;
use std::thread;

mod run;
mod genetic_operators;


macro_rules! get_arg {
    ($matches: expr, $x:expr, $type: ident) => {
        $matches
            .value_of($x)
            .unwrap()
            .parse::<$type>()
            .expect(&format!("Invalid value for {}", $x));
    };
}

fn main() {
    let matches = App::new("GA run")
        .about("Run the Genetic Algorithm")
        .arg(
            Arg::with_name("n")
                .help("log₂ of N, the height of the OA. (N = 2ⁿ)")
                .required(true),
        )
        .arg(
            Arg::with_name("k")
                .help("the width of the OA")
                .required(true),
        )
        .arg(
            Arg::with_name("t")
                .help("the strength of the OA")
                .required(true),
        )
        .arg(
            Arg::with_name("epochs")
                .help("Number of epochs per run")
                .long("epochs")
                .default_value("10000"),
        )
        .arg(
            Arg::with_name("pop-size")
                .long("pop-size")
                .help("The size of the population")
                .default_value("50"),
        )
        .arg(
            Arg::with_name("mutation-prob")
                .long("mutation-prob")
                .help("The probability that the offspring is mutated")
                .default_value("0.2"),
        )
        .arg(
            Arg::with_name("breed-factor")
                .long("breed-factor")
                .help("Fraction of breeders (the most fit will be chosen) in the total population ")
                .default_value("0.2"),
        )
        .arg(
            Arg::with_name("survival-factor")
                .long("survival-factor")
                .help("Fractions of individuals who will survive to the next epoch")
                .default_value("0.8"),
        )
        .arg(
            Arg::with_name("runs")
                .long("runs")
                .help("Number of runs in the campaign")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("log")
                .long("log")
                .help("The results of the campaign will be written to this file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .help("The max number of runs to be done in parallel")
                .default_value("1"),
        )
        .get_matches();

    let n = get_arg!(matches, "n", usize);
    let params = run::RunParameters {
        n,
        k: get_arg!(matches, "k", usize),
        t: get_arg!(matches, "t", u32),
        epochs: get_arg!(matches, "epochs", usize),
        pop_size: get_arg!(matches, "pop-size", usize),
        mutation_prob: get_arg!(matches, "mutation-prob", f64),
        breed_factor: get_arg!(matches, "breed-factor", f64),
        survival_factor: get_arg!(matches, "survival-factor", f64),
    };
    let runs = get_arg!(matches, "runs", usize);
    let threads = get_arg!(matches, "threads", usize);
    let log = matches.value_of("log");

    let termlogger = SimpleLogger::new(LevelFilter::Info, Config::default());
    if log.is_some() {
        CombinedLogger::init(vec![
            termlogger,
            WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                File::create(log.unwrap()).unwrap(),
            ),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![termlogger]).unwrap();
    }

    info!(
        "Looking for OA[N: {}, k: {}, s: 2, t: {}]",
        2usize.pow(params.n as u32),
        params.k,
        params.t
    );
    debug!("{:#?}", params);

    let show_progress = threads == 1;
    let runs_per_thread = runs / threads;
    let resto = runs % threads;
    let join_handles: Vec<_> = (0..threads)
        .map(|thr| {
            thread::spawn(move || {
                debug!("Starting thread {}",thr);
                let mut my_finds = 0usize;
                let my_runs = if thr < resto {
                    runs_per_thread + 1
                } else {
                    runs_per_thread
                };
                for run_n in 0..my_runs {
                    if run::run(&params, show_progress) {
                        my_finds += 1;
                        info!("Found OA ({}:{})", thr, run_n);
                    } else {
                        info!("Not found ({}:{})", thr, run_n);
                    }
                }
                my_finds
            })
        })
        .collect();
    let mut found = 0;
    for j in join_handles {
        found += j.join().unwrap();
    }
    info!("Found {} suitable OA in {} runs: {}%", found, runs, found as f64 /runs as f64 * 100.0);
}
