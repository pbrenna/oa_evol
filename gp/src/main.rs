extern crate ctrlc;
extern crate evco;
extern crate oarray;
extern crate pbr;
extern crate rand;
extern crate spiril;

use pbr::ProgressBar;
use spiril::unit::Unit;

use ctrlc::set_handler;
use evco::gp::*;
use rand::OsRng;
use spiril::population::Population;
use std::f64;
use std::sync::mpsc;

mod gpoarray;
mod treeformula;
use gpoarray::GPOArray;

fn main() {
    let n = 4; //N = 2^n
    let k = 15;
    let t = 2;
    let pop_size = 500;
    println!("Looking for OA({}, {}, 2, {})", 2usize.pow(n as u32), k, t);

    let max_depth = n;
    //let leaf_bias = 0.8;
    let mutation_prob = 0.5;
    let crossover = Crossover::hard_prune(max_depth);
    //let crossover = Crossover::one_point_leaf_biased(leaf_bias);
    let mutation = Mutation::uniform();
    let epochs = 1000;
    let epoch = spiril::epoch::DefaultEpoch::new(0.2, 0.8);

    //let mut rng = OsRng::new().unwrap();
    //let tree_gen = TreeGen::full(&mut rng, 1, 4);

    let rng = OsRng::new().unwrap();
    let population: Vec<GPOArray<_>> = (0..pop_size)
        .map(|_| {
            GPOArray::new_rand(
                n,
                k,
                t,
                max_depth,
                rng.clone(),
                crossover,
                mutation,
                mutation_prob,
            )
        })
        .collect();

    let mut pb = ProgressBar::new(epochs);

    let (tx, rx) = mpsc::channel();
    set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();

    let f = Population::new(population)
        .set_size(pop_size)
        .register_callback(Box::new(move |i, j| {
            pb.message(&format!(" Best: {:.4}, Mean: {:.4}; iteration ", i, j));
            (&mut pb).inc();
            if -i < f64::EPSILON {
                return false;
            }
            rx.try_recv().is_err()
        }))
        .epochs_parallel(epochs as u32, 4, &epoch) // 4 CPU cores
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap())
        .unwrap();
    if -asd.fitness() < f64::EPSILON {
        println!("\n\n{}\n\n{}\n\n", asd.to_oarray(), asd);
        std::process::exit(2);
    } else {
        std::process::exit(1);
    }
}
