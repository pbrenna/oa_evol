use spiril::{population::Population, unit::Unit};
//use std::cmp;
use std::f64;

//mod epoch;
use epoch::TournamentEpoch;

use genetic_operators::{generate_partial, IncGAOArray};
use oarray::FitnessFunction;
use pbr::ProgressBar;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RunParameters {
    pub ngrande: usize,
    pub k: usize,
    pub t: u32,
    pub pop_size: usize,
    pub mutation_prob: f64,
    pub epochs: usize,
    pub fitness_f: FitnessFunction,
}

pub(crate) fn run(
    p: &RunParameters,
    show_progress: bool,

) -> (bool, bool) {
    let ngrande = p.ngrande;
    assert!(ngrande % (2usize.pow(p.t)) == 0, "2^t non divide N");
    let mut partial = generate_partial(ngrande, p.t, p.fitness_f);
    let mut k_current = p.t as usize;
    let epoch = TournamentEpoch::new();
    while k_current < p.k {
        let num_epochs = p.epochs * (k_current + 1 - p.t as usize);
        let best;
        let mut cnt = 0;
        {
            let mut units: Vec<IncGAOArray> = Vec::with_capacity(p.pop_size);
            for _ in 0..p.pop_size {
                units.push(IncGAOArray::new(&partial, p.mutation_prob, p.k));
            }

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
                }))
                .epochs(num_epochs as u32, &epoch)
                .finish();
            //we have a suitable N * k_current array

            best = f
                .iter()
                .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap())
                .unwrap()
                .complete_oa();
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
        (true, partial.check_linear())
    } else {
        println!("ASD1");
        (false, false)
    }
}

/*fn oa_cmp<'r, 's>(a: &'r IncGAOArray<'r>, b: &'s IncGAOArray<'s>) -> cmp::Ordering {
    a.fitness().partial_cmp(&b.fitness()).unwrap()
}*/
