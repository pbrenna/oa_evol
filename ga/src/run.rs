use spiril::{population::Population, unit::Unit, epoch::DefaultEpoch};
use std::f64;

//mod epoch;
//use epoch::TournamentEpoch;

use genetic_operators::GAOArray;
use oarray::OArray;
use rand::thread_rng;
use pbr::ProgressBar;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RunParameters {
    pub n: usize,
    pub k: usize,
    pub t: u32,
    pub pop_size: usize,
    pub mutation_prob: f64,
    pub breed_factor: f64,
    pub survival_factor: f64,
    pub epochs: usize,
}

pub(crate) fn run(p: &RunParameters, show_progress: bool) -> bool {
    let mut rng = thread_rng();
    let ngrande = 2usize.pow(p.n as u32);
    let units: Vec<GAOArray> = (0..p.pop_size)
        .map(|_i| GAOArray {
            oa: OArray::new_random_balanced(ngrande, p.k, p.t, &mut rng),
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
        true
    } else {
        false
    }
}
