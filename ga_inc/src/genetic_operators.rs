use oarray::t_combinations;
use oarray::FitnessFunction::{self, *};
use oarray::OArray;
use rand::thread_rng;
use rand::Rng;
use spiril::unit::Unit;
use std::f64;
use std::iter::repeat;
use streaming_iterator::StreamingIterator;

/// Istanza di OArray dedicata all'algoritmo genetico: implementa
/// la mutazione.
#[derive(Clone)]
pub struct IncGAOArray<'a> {
    pub partial: &'a OArray,
    pub last_col: Vec<bool>,
    pub mutation_prob: f64,
    pub target_k: usize,
}

impl<'a> IncGAOArray<'a> {
    pub fn new(partial: &'a OArray, mutation_prob: f64, target_k: usize) -> Self {
        let mut r = thread_rng();
        assert!(-partial.fitness() < f64::EPSILON);
        let mut last_col: Vec<bool> = [true, false]
            .iter()
            .cloned()
            .cycle()
            .take(partial.ngrande)
            .collect();
        r.shuffle(&mut last_col);
        IncGAOArray {
            partial,
            last_col,
            mutation_prob,
            target_k,
        }
    }

    /// Scambia due coordinate nel vettore con probabiltà `prob`,
    /// usando `rng`.
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let n = self.partial.ngrande;
        if rng.gen_range::<f64>(0.0, 1.0) < prob {
            //pick random coordinate to mutate
            let coord1 = rng.gen_range(0, n);
            //pick other coordinate to swap
            let mut coord2 = rng.gen_range(0, n);
            while self.last_col[coord2] == self.last_col[coord1] {
                coord2 = rng.gen_range(0, n);
            }
            self.last_col.swap(coord1, coord2);
        }
    }
    pub fn complete_oa(&self) -> OArray {
        let mut other = self.partial.clone();
        other.d.append(&mut self.last_col.clone());
        other.k += 1;
        other
    }
    fn walsh_furba_faster(&self, exp: u32) -> f64 {
        let cols: Vec<&[bool]> = self.partial.iter_cols().collect();
        let mut grand_tot = self
            .last_col
            .iter()
            .map(|&i| if i { -1 } else { 1 })
            .sum::<i64>()
            .pow(exp)
            .abs();
        let tmp0 = self.last_col.clone();
            t_combinations::combinations_descent(
            self.partial.k,
            self.partial.target_t as usize - 1,
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
    fn walsh_furba(&self, exp: u32) -> f64 {
        //per tutte le combinazioni che non includono last_col, la fitness deve
        //essere 0
        let mut grand_tot = self
            .last_col
            .iter()
            .map(|&i| if i { -1 } else { 1 })
            .sum::<i64>()
            .pow(exp)
            .abs();
        let rows: Vec<Vec<&bool>> = self.partial.iter_rows().collect();
        for w in 1..self.partial.target_t {
            let mut combs = t_combinations::Combinations::new(self.partial.k, w);
            let mut comb_iter = combs.stream_iter();
            while let Some(comb) = comb_iter.next() {
                let mut vec_tot = 0i64;
                for (j, u) in rows.iter().enumerate() {
                    let prod = comb.iter().map(|i| u[*i]).fold(false, |acc, cur| acc ^ cur)
                        ^ self.last_col[j];
                    vec_tot += if prod { -1 } else { 1 };
                }
                //println!("{}, {:?}", vec_tot, comb);
                grand_tot += vec_tot.pow(exp).abs();
            }
        }
        -grand_tot as f64
    }
}
/// Unisce due OArray in in modo che il risultato sia bilanciato
fn balanced_crossover(a: &[bool], b: &[bool], out: &mut [bool], r: &mut impl Rng) {
    let ngrande = a.len();
    let balance = ngrande / 2;
    let mut pos: Vec<_> = (0..ngrande).collect();
    r.shuffle(&mut pos);
    let mut cnt = [balance, balance];
    for j in pos {
        let choice = if cnt[0] == 0 {
            true
        } else if cnt[1] == 0 {
            false
        } else {
            *r.choose(&[a[j], b[j]]).unwrap()
        };

        out[j] = choice;
        cnt[choice as usize] -= 1;
    }
}

// implement trait functions mutate and calculate_fitness:
impl<'a> Unit for IncGAOArray<'a> {
    fn breed_with(&self, other: &Self) -> Self {
        let mut rng = thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on the last col
        let mut out = self.clone();
        balanced_crossover(&self.last_col, &other.last_col, &mut out.last_col, &mut rng);
        out.mutate_with_prob(self.mutation_prob, &mut rng);
        out
    }

    fn fitness(&self) -> f64 {
        match self.partial.fitness_f {
            Walsh(x) => self.walsh_furba(x),
            WalshFaster(x) => self.walsh_furba_faster(x),
            _ => self.complete_oa().fitness(),
        }
    }
}

pub fn generate_partial(ngrande: usize, target_t: u32, fitness_f: FitnessFunction) -> OArray {
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

#[test]
fn test_furba() {
    for _ in 0..10000 {
        let p = generate_partial(16, 4, Walsh(2));
        let rnd = IncGAOArray::new(&p, 0.1, 5);
        let mut oa = rnd.complete_oa();
        let fitw = oa.fitness();
        oa.fitness_f = WalshFaster(2);
        let fitwfa = oa.fitness();

        let fitwfu = rnd.walsh_furba(2);
        let fitwfufa = rnd.walsh_furba_faster(2);
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
