use spiril::{population::Population, unit::Unit};
//use std::cmp;
use evco::gp::*;
use rand::OsRng;
use std::f64;

//mod epoch;
//use epoch::TournamentEpoch;
use spiril::epoch::DefaultEpoch;

use gpoarray::IncGPOArray;
use oarray::{FitnessFunction, OArray};
use pbr::ProgressBar;
//use treeformula::TreeFormula;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RunParameters {
    pub ngrande: usize,
    pub k: usize,
    pub t: u32,
    pub pop_size: usize,
    pub mutation_prob: f64,
    pub max_depth: usize,
    pub epochs: usize,
    pub fitness_f: FitnessFunction,
}

pub(crate) fn run(p: &RunParameters, show_progress: bool) -> (bool, bool) {
    let ngrande = p.ngrande;
    assert!(ngrande % (2usize.pow(p.t)) == 0, "2^t non divide N");
    let mut partial = OArray::generate_partial(ngrande, p.t, p.fitness_f);
    let mut k_current = p.t as usize;
    //let epoch = TournamentEpoch::new();
    let epoch = DefaultEpoch::default();
    let crossover = Crossover::hard_prune(p.max_depth);
    //let crossover = Crossover::one_point_leaf_biased(leaf_bias);
    //let mutation = Mutation::uniform_prune(p.max_depth);
    let mutation= Mutation::uniform();
    let rng = OsRng::new().unwrap();
    let mut formulas = Vec::new();
    while k_current < p.k {
        let num_epochs = p.epochs * (k_current + 1 - p.t as usize);
        let best;
        let mut cnt = 0;
        {
            let units: Vec<IncGPOArray<_>> = (0..p.pop_size)
                .map(|_| {
                    IncGPOArray::new_rand(
                        &partial,
                        p.ngrande,
                        p.k,
                        p.t,
                        p.max_depth,
                        rng.clone(),
                        crossover,
                        mutation,
                        p.mutation_prob,
                        p.fitness_f,
                    )
                }).collect();

            let mut pbar = ProgressBar::new(num_epochs as u64);

            let f = Population::new(units)
                .set_size(p.pop_size)
                .register_callback(Box::new(move |i, j| {
                    if show_progress && cnt % 100 == 0 {
                        pbar.message(&format!(
                            " Col: {}, Best: {:.4}, Mean: {:.4}; iteration ",
                            k_current, i, j
                        ));
                        (&mut pbar).set(cnt);
                    }
                    cnt += 1;
                    if -i < f64::EPSILON {
                        return false;
                    }
                    true
                })).epochs(num_epochs as u32, &epoch)
                .finish();
            //we have a suitable N * k_current array

            let best1 = f
                .iter()
                .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap())
                .unwrap();
            formulas.push(best1.tree.clone());
            best = best1.to_oarray();
        }
        if -best.fitness() < f64::EPSILON {
            //println!("{:?}", &best.d[best.ngrande * (best.k - 1)..]);

            partial = best;
        } else {
            return (false, false);
        }
        k_current = partial.k;
    }
    if -partial.fitness() < f64::EPSILON && partial.k == p.k {
        debug!("{}", partial);
        for (i, f) in formulas.iter().enumerate() {
            debug!("Formula {}: {}", i + 1, f);
        }
        (true, partial.check_linear())
    } else {
        println!("ASD1");
        (false, false)
    }
}

/*fn oa_cmp<'r, 's>(a: &'r IncGAOArray<'r>, b: &'s IncGAOArray<'s>) -> cmp::Ordering {
    a.fitness().partial_cmp(&b.fitness()).unwrap()
}*/
