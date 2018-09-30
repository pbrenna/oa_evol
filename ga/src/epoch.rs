use rand::Rng;
use spiril::{epoch::Epoch, population::LazyUnit, unit::Unit};
use std::cmp::Ordering;

pub struct TournamentEpoch {}
impl TournamentEpoch {
    pub fn new() -> Self {
        TournamentEpoch { }
    }
}

impl<T: Unit + Clone> Epoch<T> for TournamentEpoch {
    fn epoch(&self, units: &mut Vec<LazyUnit<T>>, size: usize, r: &mut impl Rng) -> bool {
        let cmp_func = |a: &&LazyUnit<T>, b: &&LazyUnit<T>| {
            a.lazy_fitness
                .partial_cmp(&b.lazy_fitness)
                .unwrap_or(Ordering::Equal)
        };
        let unit_len = units.len();
        assert!(units.len() >= 3);
        let mut new_vec = Vec::with_capacity(size);
        for _ in 0..size {
            let a = r.gen_range(0, unit_len);
            let mut b = a;
            while b == a {
                b = r.gen_range(0, unit_len);
            }
            let mut c = b;
            while c == b || c == a {
                c = r.gen_range(0, unit_len);
            }
            let mut tmp = [&units[a], &units[b], &units[c]];
            tmp.sort_by(cmp_func);
            //println!("{:?},{:?},{:?}",tmp[0].lazy_fitness,tmp[1].lazy_fitness,tmp[2].lazy_fitness);
            let child = tmp[2].unit.breed_with(&tmp[1].unit);
            new_vec.push(LazyUnit::from(child));
        }
        let index = new_vec
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| cmp_func(a, b))
            .unwrap()
            .0;

        let old_best_fitness;
        {
            let best_individual = units.iter().max_by(cmp_func);
            old_best_fitness = best_individual.unwrap().lazy_fitness.unwrap_or(-1.0);

            let old_best = (best_individual.unwrap().unit).clone();
            new_vec[index] = LazyUnit::from(old_best);
        }
        std::mem::swap(&mut new_vec, units);
        old_best_fitness != 0.0
    }
}
