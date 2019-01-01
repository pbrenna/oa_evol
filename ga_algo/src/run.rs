use spiril::{population::Population, unit::Unit, epoch::DefaultEpoch};
use std::f64;

//mod epoch;
use epoch::TournamentEpoch;

use genetic_operators::GAOArray;
use oarray::{OArray, FitnessFunction};
use rand::thread_rng;
use pbr::ProgressBar;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RunParameters {
    pub ngrande: usize,
    pub k: usize,
    pub t: u32,
    pub pop_size: usize,
    pub mutation_prob: f64,
    pub breed_factor: f64,
    pub survival_factor: f64,
    pub epochs: usize,
    pub fitness_f: FitnessFunction
}

pub(crate) fn run(p: &RunParameters, show_progress: bool) -> (bool, bool) {
    let mut rng = thread_rng();
    let ngrande = p.ngrande;
    let units: Vec<GAOArray> = (0..p.pop_size)
        .map(|_i| GAOArray {
            oa: OArray::new_random_balanced(ngrande, p.k, p.t, &mut rng, p.fitness_f),
            mutation_prob: p.mutation_prob,
        })
        .collect();

    let mut pbar = ProgressBar::new(p.epochs as u64);

    //let epoch = TournamentEpoch::new();
    let epoch = DefaultEpoch::new(p.breed_factor, p.survival_factor);
    let f = Population::new(units)
        .set_size(p.pop_size)
        .register_callback(Box::new(move |i, j| {
            if show_progress {
                pbar.message(&format!(" Best: {:.4}, Mean: {:.4}; iteration ", i, j));
                (&mut pbar).inc();
            }
            if -i < f64::EPSILON {
                return false;
            }
            true
        }))
        .epochs(p.epochs as u32, &epoch)
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    if -asd.unwrap().fitness() < f64::EPSILON {
        debug!("{}", asd.unwrap().oa);
        (true, asd.unwrap().oa.check_linear())
    } else {
        (false, false)
    }
}
