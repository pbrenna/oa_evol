use oarray::OArray;
use streaming_iterator::StreamingIterator;
use t_combinations::Combinations;

#[derive(Clone, Debug, Copy)]
pub enum FitnessFunction {
    Delta,
    DeltaFast,
    Walsh(u32), //exponent
}
pub use self::FitnessFunction::*;

impl OArray {
    fn delta_fitness(&self) -> f64 {
        let mut comb = Combinations::new(self.k, self.target_t);
        let asd: f64 = comb
            .stream_iter()
            .map(|igrande| self.delta_grande(&igrande, 2.0))
            .cloned()
            .sum();
        -asd
    }
    fn delta_fitness_fast(&self) -> f64 {
        let mut comb = Combinations::new(self.k, self.target_t);
        let asd: f64 = comb
            .stream_iter()
            .map(|igrande| self.delta_grande_faster(&igrande, 2))
            .cloned()
            .sum();
        -asd
    }
    pub fn fitness(&self) -> f64 {
        match self.fitness_f {
            Delta => self.delta_fitness(),
            DeltaFast => self.delta_fitness_fast(),
            Walsh(exponent) => self.walsh_fitness(exponent),
        }
    }

    /// conta il numero di occorrenze di `needle` nelle colonne `igrande` dell'array,
    /// e restituisce la differenza rispetto al livello `lambda`
    fn delta(&self, igrande: &[usize], needle: usize, lambda: usize) -> usize {
        let mut out = 0;
        for i in 0..self.ngrande {
            //iterate rows
            let cur_row = igrande.iter().fold(0, |acc, col| {
                (acc << 1) | (self.d[col * self.ngrande + i] as usize)
            });
            if cur_row == needle {
                out += 1
            }
        }
        (lambda as isize - out as isize).abs() as usize
    }

    /// calcola per ogni numero rappresentabile da `igrande.len` bit
    /// la funzione delta, usa i risultati per dare una distanza.
    fn delta_grande(&self, igrande: &[usize], p: f64) -> f64 {
        let t_num = igrande.len();
        let num_representable_strings = 2usize.pow(t_num as u32);
        let lambda = self.ngrande / num_representable_strings;
        (0..num_representable_strings) //last is excluded
            .map(|i| {
                let d = self.delta(igrande, i, lambda);
                (d as f64).powf(p)
            })
            .sum::<f64>()
            .powf(1.0 / p)
    }

    fn delta_grande_faster(&self, igrande: &[usize], p: u32) -> f64 {
        let t_num = igrande.len();
        let num_representable_strings = 2usize.pow(t_num as u32);
        let lambda = self.ngrande / num_representable_strings;
        let mut counts = vec![lambda as i64; num_representable_strings];
        for i in 0..self.ngrande {
            let cur_row = igrande.iter().fold(0, |acc, col| {
                acc * 2 + (self.d[col * self.ngrande + i] as usize)
            });
            counts[cur_row] -= 1;
        }
        let tot: i64 = counts.iter().map(|&i| i.abs().pow(p)).sum();
        (tot as f64).powf(1.0 / f64::from(p))
    }
    /// Walsh
    fn walsh_fitness(&self, exp: u32) -> f64 {
        let t = self.target_t;
        let mut grand_tot = 0;
        let rows: Vec<Vec<&bool>> = self.iter_rows().collect();
        for w in 1..=t {
            let mut combs = Combinations::new(self.k, w);
            let mut comb_iter = combs.stream_iter();
            while let Some(comb) = comb_iter.next() {
                let mut vec_tot = 0i64;
                for u in &rows {
                    let prod = comb.iter().map(|i| u[*i]).fold(false, |acc, cur| acc ^ cur);
                    vec_tot += if prod { -1 } else { 1 };
                }
                grand_tot += vec_tot.pow(exp).abs();
            }
        }
        -grand_tot as f64
    }
}

#[allow(unused_macros)]
macro_rules! bool_vec {
    ($($x:expr),*) => {
        vec![$($x != 0),*]
    };
}

#[cfg(test)]
mod test {
    use OArray;
    use FitnessFunction::*;
    use rand::thread_rng;
    use std::f64::EPSILON;
    #[test]
    fn check_fitness1() {
        let test = OArray::new(4, 2, 2, bool_vec![0, 0, 1, 1, 0, 1, 0, 1], DeltaFast);
        assert!(test.delta_fitness_fast() == 0.0);
        assert!(test.walsh_fitness(2) == 0.0);
    }

    #[test]
    fn check_fast_delta() {
        use rand::thread_rng;
        use std::f64::EPSILON;
        let mut rng = thread_rng();
        let error = EPSILON;
        for _ in 0..100 {
            let rand = OArray::new_random_balanced(8, 7, 3, &mut rng, DeltaFast);
            assert!((rand.delta_fitness() - rand.delta_fitness_fast()).abs() < error);
        }
    }

    #[test]
    fn check_fitness2() {
        let test = OArray::new(4, 2, 1, bool_vec![0, 1, 1, 1, 0, 1, 0, 1], DeltaFast);
        assert!(test.delta_fitness_fast() != 0.0);
        assert!(test.walsh_fitness(2) != 0.0);
    }

    #[test]
    fn test_walsh() {
        let mut rng = thread_rng();
        let error = EPSILON;
        for _ in 0..1000 {
            let rand = OArray::new_random_balanced(8, 7, 3, &mut rng, DeltaFast);
            let delta_is_zero = -rand.delta_fitness_fast() < error;
            let walsh_is_zero = -rand.walsh_fitness(2) < error;
            assert!(delta_is_zero == walsh_is_zero);
        }
    }
}
