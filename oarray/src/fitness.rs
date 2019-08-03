use oarray::OArray;
use streaming_iterator::StreamingIterator;
use t_combinations::{combinations_descent, Combinations};
use wtform::PolarTruthTable;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FitnessFunction {
    Delta,
    DeltaFast,
    Walsh(u32),       //exponent
    WalshFaster(u32), //exponent
    WalshRec(u32),
    Cidev,
    SheerLuck,
    Comb(u32),
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
        let ret = match self.fitness_f {
            Delta => self.delta_fitness(),
            DeltaFast => self.delta_fitness_fast(),
            Walsh(exponent) => self.walsh_fitness(exponent),
            WalshFaster(exponent) => self.walsh_faster(exponent),
            WalshRec(exponent) => self.walsh_fitness_rec(exponent as f64),
            Cidev => self.cidev_fitness(),
            SheerLuck => self.sheer_luck_fitness(),
            Comb(exponent) => self.comb_fitness(exponent as f64),
        };
        //dbg!(ret);
        debug_assert!(ret <= 0.0, "overflow");
        ret
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

    pub(crate) fn delta_grande_faster(&self, igrande: &[usize], p: u32) -> f64 {
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
    #[inline(never)]
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
                //println!("{}, {:?}", vec_tot, comb);
                grand_tot += vec_tot.pow(exp).abs();
            }
        }
        -grand_tot as f64
    }
    #[inline(never)]
    fn walsh_faster(&self, exp: u32) -> f64 {
        let cols: Vec<&[bool]> = self.iter_cols().collect();
        let mut grand_tot = 0;
        let tmp0 = vec![false; self.ngrande];
        combinations_descent(self.k, self.target_t as usize, 0, &tmp0, &mut |i, tmp| {
            let tmp1: Vec<bool> = cols[i].iter().zip(tmp.iter()).map(|(a, b)| a ^ b).collect();
            let mut my_tot = 0i64;
            for x in &tmp1 {
                my_tot += if *x { 1 } else { -1 };
            }
            //println!("{:?}, {:?}", my_tot, tmp1);
            grand_tot += my_tot.pow(exp).abs();
            tmp1
        });
        -grand_tot as f64
    }
    fn walsh_fitness_rec(&self, p: f64) -> f64 {
        let k = self.k;
        -recurse_comb(
            k,
            self.target_t as usize,
            1,
            vec![false; self.ngrande],
            self,
            p,
        )
    }
    fn cidev_fitness(&self) -> f64 {
        let tt = self.truth_table();
        let ptt = PolarTruthTable::from(&tt);
        let wtf = ptt.walsh_tform();
        let cidev = wtf.cidev(self.target_t as usize);
        -f64::from(cidev)
    }
    fn sheer_luck_fitness(&self) -> f64 {
        if self.delta_fitness_fast() > -std::f64::EPSILON {
            0.0
        } else {
            -1.0
        }
    }
    fn comb_fitness(&self, p: f64) -> f64 {
        let a = -self.walsh_fitness_rec(2.0);
        let b = PolarTruthTable::from(&self.truth_table()).walsh_tform().nonlinearity();
        -(a + f64::max(0.0, p * (self.k as f64)-1.0-b))
    }
}
pub(crate) fn walsh_step(
    agrande: &OArray,
    i: usize,
    column: Vec<bool>,
    p: f64,
) -> (Vec<bool>, f64) {
    let mut total = 0i64;
    let mut new_column = vec![false; agrande.ngrande];
    for j in 1..=agrande.ngrande {
        new_column[j - 1] = column[j - 1] ^ agrande.iter_cols().nth(i - 1).unwrap()[j - 1];
        if new_column[j - 1] == true {
            total = total + 1;
        } else {
            total = total - 1;
        }
        //        dbg!(total);
    }
    (new_column, (total as f64).abs().powf(p))
}

pub(crate) fn recurse_comb(
    k: usize,
    comb_len: usize,
    base: usize,
    column: Vec<bool>,
    agrande: &OArray,
    p: f64,
) -> f64 {
    let mut total = 0.0;
    if comb_len < 1 {
        return 0.0;
    }
    for i in base..=k {
        let (new_column, partial) = walsh_step(agrande, i, column.clone(), p);
        total = total + partial;
        let rec = recurse_comb(k, comb_len - 1, i + 1, new_column, agrande, p);
        total = total + rec;
    }
    total
}

#[allow(unused_macros)]
macro_rules! bool_vec {
    ($($x:expr),*) => {
        vec![$($x != 0),*]
    };
}

#[cfg(test)]
mod test {
    use rand::thread_rng;
    use std::f64::EPSILON;
    use FitnessFunction::*;
    use OArray;
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
    #[test]
    fn test_walsh_faster() {
        let mut rng = thread_rng();
        let error = EPSILON;
        for _ in 0..1000 {
            let rand = OArray::new_random_balanced(8, 7, 3, &mut rng, DeltaFast);
            assert!((rand.walsh_faster(2) - rand.walsh_fitness(2)).abs() < error);
        }
    }
    #[test]
    fn test_paper_impl() {
        let mut rng = thread_rng();
        let error = EPSILON;
        for _ in 0..1000 {
            let rand = OArray::new_random_balanced(16, 15, 3, &mut rng, DeltaFast);
            let a = rand.walsh_fitness_rec(2.0);
            let b = rand.walsh_fitness(2);
            assert!((a - b).abs() < error);
        }
    }
    /* mod bench {
        use rand::thread_rng;
        use std::f64::EPSILON;
        use test::Bencher;
        use FitnessFunction::*;
        use OArray;

        #[bench]
        fn bench_walsh2(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 2;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh3(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 3;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh4(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 4;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh5(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 5;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh6(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 6;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh7(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 7;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_walsh8(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 8;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, Walsh(2));
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast2(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 2;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast3(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 3;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast4(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 4;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast5(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 5;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast6(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 6;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast7(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 7;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
        #[bench]
        fn bench_delta_fast8(b: &mut Bencher) {
            let mut rng = thread_rng();
            let t = 8;
            let oa = OArray::new_random_balanced(256, 20, t, &mut rng, DeltaFast);
            b.iter(|| {
                let b = oa.fitness();
                test::black_box(b);
            })
        }
    }*/
}
