extern crate ctrlc;
extern crate num_iter;
extern crate pbr;
extern crate rand;
extern crate spiril;
extern crate oarray;

use spiril::{population::Population, unit::Unit};
use epoch::TournamentEpoch;

mod epoch;
mod genetic_operators;

use oarray::OArray;
use genetic_operators::GAOArray;


//Parametri di esecuzione
const N: usize = 16;
const K: usize = 8;
const S: u8 = 2;
const T: usize = 2;


fn main() {
    let n_units = 50;
    let epochs = 10000;
    let mut rng = rand::thread_rng();

    println!("Looking for OA({},{},{},{})", N, K, S, T);
    let units: Vec<GAOArray<_>> = (0..n_units)
        .map(|_i| GAOArray(OArray::new_random_balanced(N, K, S, T, &mut rng)))
        .collect();

    let mut pbar = pbr::ProgressBar::new(epochs);
    let (tx, rx) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    }).expect("Can't register ctrl+c");

    let epoch = TournamentEpoch::new();
    let epoch = spiril::epoch::DefaultEpoch::new(0.2, 0.8);
    let f = Population::new(units)
        .set_size(n_units)
        .set_breed_factor(0.2)
        .set_survival_factor(0.8)
        .register_callback(Box::new(move |i, j| {
            pbar.message(&format!(" Best: {:.4}, Mean: {:.4}; iteration ", i, j));
            /*for x in units {
                println!("{}", x.unit);
            }*/
            (&mut pbar).inc();
            rx.try_recv().is_err()
        })).epochs_parallel(epochs as u32, 4, &epoch) // 4 CPU cores
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    if -asd.unwrap().fitness() < std::f64::EPSILON {
        println!("\n\n{}\n\n", asd.unwrap().0);
        std::process::exit(2);
    } else {
        std::process::exit(1);
    }
}
