use fitness::FitnessFunction;
use oarray::OArray;
use std::iter::repeat;
use streaming_iterator::StreamingIterator;
use t_combinations;

impl OArray {
    pub fn generate_partial(ngrande: usize, target_t: u32, fitness_f: FitnessFunction) -> Self {
        let stringhe = 2usize.pow(target_t);
        let lambda = ngrande / stringhe;
        assert!(lambda * stringhe == ngrande);
        let mut d = vec![];
        for col in 0..target_t {
            let num = 2usize.pow(col);
            let base = repeat(false).take(num).chain(repeat(true).take(num));
            let mut dati_colonna = base.cycle().take(ngrande).collect();
            d.append(&mut dati_colonna);
        }
        OArray::new(ngrande, target_t as usize, target_t, d, fitness_f)
    }
    /**
     * self: a valid OA except for the last column 
     * */
    pub fn delta_incremental_faster(&self) -> f64 {
        let mut comb = t_combinations::Combinations::new(self.k-1, self.target_t - 1);
        let out: f64 = comb
            .stream_iter()
            .map(|igrande| {
                let mut my_igrande = Vec::new();
                my_igrande.push(self.k-1);
                my_igrande.extend_from_slice(igrande);
                self.delta_grande_faster(&my_igrande, 2)
                })
            .cloned()
            .sum();
        -out
    }

    pub fn walsh_incremental_faster(&self, exp: u32, last: &[bool]) -> f64 {
        let cols: Vec<&[bool]> = self.iter_cols().collect();
        let mut grand_tot = last
            .iter()
            .map(|&i| if i { -1 } else { 1 })
            .sum::<i64>()
            .pow(exp)
            .abs();
        let tmp0 = Vec::from(last);
        t_combinations::combinations_descent(
            self.k,
            self.target_t as usize - 1,
            0,
            0,
            &tmp0,
            &mut |i, tmp| {
                let tmp1: Vec<bool> = cols[i].iter().zip(tmp.iter()).map(|(a, b)| a ^ b).collect();
                let mut my_tot = 0i64;
                for x in &tmp1 {
                    my_tot += if *x { 1 } else { -1 };
                }
                //println!("{:?}, {:?}", my_tot, tmp1);
                grand_tot += my_tot.pow(exp).abs();
                tmp1
            },
        );
        -grand_tot as f64
    }
    pub fn walsh_incremental(&self, exp: u32, last: &[bool]) -> f64 {
        //per tutte le combinazioni che non includono last_col, la fitness deve
        //essere 0
        let mut grand_tot = last
            .iter()
            .map(|&i| if i { -1 } else { 1 })
            .sum::<i64>()
            .pow(exp)
            .abs();
        let rows: Vec<Vec<&bool>> = self.iter_rows().collect();
        for w in 1..self.target_t {
            let mut combs = t_combinations::Combinations::new(self.k, w);
            let mut comb_iter = combs.stream_iter();
            while let Some(comb) = comb_iter.next() {
                let mut vec_tot = 0i64;
                for (j, u) in rows.iter().enumerate() {
                    let prod =
                        comb.iter().map(|i| u[*i]).fold(false, |acc, cur| acc ^ cur) ^ last[j];
                    vec_tot += if prod { -1 } else { 1 };
                }
                //println!("{}, {:?}", vec_tot, comb);
                grand_tot += vec_tot.pow(exp).abs();
            }
        }
        -grand_tot as f64
    }
}

#[test]
fn test_incremental() {
    use rand::thread_rng;
    use rand::Rng;
    use std::f64;
    use FitnessFunction::*;
    let mut r = thread_rng();
    for _ in 0..10000 {
        let partial = OArray::generate_partial(16, 4, Walsh(2));
        let mut last: Vec<bool> = [true, false]
            .iter()
            .cloned()
            .cycle()
            .take(partial.ngrande)
            .collect();
        r.shuffle(&mut last);
        let mut oa = partial.clone();
        oa.d.append(&mut last.clone());
        oa.k += 1;
        let fitw = oa.fitness();
        oa.fitness_f = WalshFaster(2);
        let fitwfa = oa.fitness();

        let fitwfu = partial.walsh_incremental(2, &last);
        let fitwfufa = partial.walsh_incremental_faster(2, &last);
        let error = f64::EPSILON;
        assert!(
            (fitw - fitwfa).abs() < error
                && (fitwfa - fitwfu).abs() < error
                && (fitwfu - fitwfufa).abs() < error,
            "{}, {}, {}, {}",
            fitw,
            fitwfa,
            fitwfu,
            fitwfufa
        );
    }
}


#[test]
fn test_delta_incremental() {
    use rand::thread_rng;
    use rand::Rng;
    use std::f64;
    use FitnessFunction::*;
    let mut r = thread_rng();
    for _ in 0..10000 {
        let partial = OArray::generate_partial(16, 4, DeltaFast);
        let mut last: Vec<bool> = [true, true, false]
            .iter()
            .cloned()
            .cycle()
            .take(partial.ngrande)
            .collect();
        r.shuffle(&mut last);
        let mut oa = partial.clone();
        oa.d.append(&mut last.clone());
        oa.k += 1;
        let fit_delta = oa.fitness();

        let fit_delta_partial = oa.delta_incremental_faster();
        let error = f64::EPSILON;
        assert!((fit_delta - fit_delta_partial).abs() < error,
            "{}, {}",
            fit_delta,
            fit_delta_partial,
        );
    }
}
