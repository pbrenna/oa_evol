
use evco::gp::*;
use pbr::ProgressBar;
use rand::OsRng;
use spiril::population::Population;
use spiril::unit::Unit;
use std::f64;
use gpoarray::GPOArray;
use spiril::epoch::DefaultEpoch;
use oarray::FitnessFunction;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RunParameters {
    pub n : usize,
    pub k : usize, 
    pub t : u32,
    pub pop_size : usize,
    pub max_depth: usize,
    pub mutation_prob: f64,
    pub breed_factor: f64,
    pub survival_factor: f64,
    pub epochs: usize,
    pub fitness_f: FitnessFunction
}


pub(crate) fn run(p : &RunParameters, show_progress: bool) -> (bool, bool) {

    let crossover = Crossover::hard_prune(p.max_depth);
    //let crossover = Crossover::one_point_leaf_biased(leaf_bias);
    let mutation = Mutation::uniform();
    let epoch = DefaultEpoch::new(0.2, 0.8);

    //let mut rng = OsRng::new().unwrap();
    //let tree_gen = TreeGen::full(&mut rng, 1, 4);

    let rng = OsRng::new().unwrap();
    let population: Vec<GPOArray<_>> = (0..p.pop_size)
        .map(|_| {
            GPOArray::new_rand(
                p.n,
                p.k,
                p.t,
                p.max_depth,
                rng.clone(),
                crossover,
                mutation,
                p.mutation_prob,
                p.fitness_f
            )
        })
        .collect();

    let mut pb = ProgressBar::new(p.epochs as u64);
    //let (tx, rx) = mpsc::channel();
    /*set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();*/

    let f = Population::new(population)
        .set_size(p.pop_size)
        .register_callback(Box::new(move |i, j| {
            if show_progress {
                pb.message(&format!(" Best: {:.4}, Mean: {:.4}; iteration ", i, j));
                (&mut pb).inc();
            }
            if -i < f64::EPSILON {
                return false;
            }
            true
            //rx.try_recv().is_err()
        }))
        .epochs(p.epochs as u32, &epoch)
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap())
        .unwrap();
    if -asd.fitness() < f64::EPSILON {
        debug!("{}\n{}", asd.to_oarray(), asd);
        (true, asd.to_oarray().check_linear())
    } else {
        (false, false)
    }
}
