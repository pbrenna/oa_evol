use spiril::{epoch::DefaultEpoch, population::Population, unit::Unit};
//use std::cmp;
use std::f64;

//mod epoch;
//use epoch::TournamentEpoch;

use genetic_operators::{generate_partial, IncGAOArray};
use oarray::FitnessFunction;
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
    pub fitness_f: FitnessFunction,
}

pub(crate) fn run(p: &RunParameters, show_progress: bool) -> (bool, bool) {
    let ngrande = 2usize.pow(p.n as u32);
    let backtrack = false;
    let mut partial = generate_partial(ngrande, p.t, p.fitness_f);
    let mut k_current = p.t as usize;
    let mut depth_stack = vec![0; p.k];
    while k_current < p.k {
        assert!(partial.d.len() == partial.ngrande * k_current);
        //println!("{}", partial);
        let max_tries = 2; //k_current - p.t as usize;
        depth_stack[k_current] += 1;
        for i in depth_stack[k_current+1 ..].iter_mut(){
            *i = 0;
        }
        let best;
        {
            let mut units: Vec<IncGAOArray> = Vec::with_capacity(p.pop_size);
            for _ in 0..p.pop_size {
                units.push(IncGAOArray::new(&partial, p.mutation_prob, p.k));
            }

            let mut pbar = ProgressBar::new(p.epochs as u64);

            //let epoch = TournamentEpoch::new();
            let epoch = DefaultEpoch::new(p.breed_factor, p.survival_factor);
            let f = Population::new(units)
                .set_size(p.pop_size)
                .register_callback(Box::new(move |i, j| {
                    if show_progress {
                        pbar.message(&format!(
                            " Col: {}, Best: {:.4}, Mean: {:.4}; iteration ",
                            k_current, i, j
                        ));
                        (&mut pbar).inc();
                    }
                    if -i < f64::EPSILON {
                        return false;
                    }
                    true
                })).epochs(p.epochs as u32, &epoch)
                .finish();
            //we have a suitable N * k_current array

            best = f
                .iter()
                .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap())
                .unwrap()
                .complete_oa();
        }
        if -best.fitness() < f64::EPSILON {
            partial = best;
        } else if depth_stack[k_current] > max_tries {
            if k_current > p.t  as usize && backtrack  {
                partial.d = Vec::from(&partial.d[0..partial.ngrande * (k_current - 1)]);
                partial.k -= 1;
            } else {
                return (false, false);
            }
        }
        k_current = partial.k;
    }
    if -partial.fitness() < f64::EPSILON && partial.k == p.k {
        debug!("{}", partial);
        (true, partial.check_linear())
    } else {
        (false, false)
    }
}

/*fn oa_cmp<'r, 's>(a: &'r IncGAOArray<'r>, b: &'s IncGAOArray<'s>) -> cmp::Ordering {
    a.fitness().partial_cmp(&b.fitness()).unwrap()
}*/
