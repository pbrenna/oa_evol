// This example implements the queens problem:
// https://en.wikipedia.org/wiki/Eight_queens_puzzle
// using an evolutionary algorithm.

extern crate rand;
extern crate simplelog;

// internal crates

extern crate itertools;
extern crate spiril;

use itertools::Itertools;
use rand::Rng;
use std::iter;

use spiril::{population::Population, unit::Unit};

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
struct OArray {
    d: Vec<bool>,
    N: usize,
    k: usize,
}

#[allow(non_upper_case_globals)]
const k: usize = 4;
const N: usize = 16;
#[allow(non_upper_case_globals)]
const t: usize = 3;

fn balanced_crossover(a: &[bool], b: &[bool], out: &mut [bool], r: &mut impl Rng) {
    let ngrande = a.len();
    let mut cnt0 = 0usize;
    let mut cnt1 = 0usize;
    let mut pos: Vec<_> = (0..ngrande).collect();
    r.shuffle(&mut pos);
    for i in pos {
        if cnt0 == ngrande / 2 {
            out[i] = true;
        } else if cnt1 == ngrande / 2 {
            out[i] = false;
        } else {
            out[i] = *r.choose(&[a[i], b[i]]).unwrap();
            if out[i] {
                cnt1 += 1;
            } else {
                cnt0 += 1;
            }
        }
    }
}

impl OArray {
    fn new_random_balanced(my_k: usize, ngrande: usize, rng: &mut impl rand::Rng) -> Self {
        assert!(
            ngrande / (2usize.pow(t as u32)) >= 1,
            "I parametri N={},s=2,t={} non soddisfano i requisiti base per un array ortogonale",
            ngrande,
            t
        );
        let mut out = OArray {
            d: iter::repeat(&[true, false])
                .map(|i| *rng.choose(i).unwrap())
                .take(ngrande * my_k)
                .collect(),
            N: ngrande,
            k: my_k,
        };
        for x in 0..k {
            rng.shuffle(&mut out.d[x * ngrande..x * ngrande + ngrande]);
        }
        out
    }

    fn delta(&self, igrande: &[usize], needle: usize, lambda: usize) -> usize {
        let mut out = 0;
        for i in 0..self.N {
            //iterate rows
            let cur_row = igrande
                .iter()
                .rev()
                .fold(0, |acc, col| acc << 1 | (self.d[col * self.N + i] as usize));
            if cur_row == needle {
                out += 1
            }
        }
        (lambda as isize - out as isize).abs() as usize
    }

    fn delta_grande(&self, igrande: &[usize], p: f64) -> f64 {
        let t_num = igrande.len();
        let max_binary = 2usize.pow(t_num as u32) - 1;
        let lambda = self.N / max_binary;
        let result = (0..max_binary)
            .map(|i| {
                let d = self.delta(igrande, i, lambda);
                (d as f64).powf(p)
            }).sum::<f64>()
            .powf(1.0 / p);
        result
    }
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let rand = rng.gen_range::<f64>(0.0, 1.0);
        if rand < prob {
            //pick random coordinate to mutate
            let coord = rng.gen_range(0, self.N * self.k);
            self.d[coord] = !self.d[coord]
        }
    }
}
// implement trait functions mutate and calculate_fitness:
impl Unit for OArray {
    fn breed_with(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on each bitstring
        let mut out = OArray {
            d: iter::repeat(false).take(self.N * self.k).collect(),
            N: self.N,
            k: self.k,
        };
        for i in 0..k {
            let ngrande = self.N;
            let col1 = &self.d[i * ngrande..i * ngrande + ngrande];
            let col2 = &other.d[i * ngrande..i * ngrande + ngrande];
            let col3 = &mut out.d[i * ngrande..i * ngrande + ngrande];
            balanced_crossover(col1, col2, col3, &mut rng);
        }
        out.mutate_with_prob(0.02, &mut rng);
        out
    }

    // fitness means here: how many queens are colliding
    fn fitness(&self) -> f64 {
        let asd: f64 = (0..self.k)
            .combinations(t)
            .map(|igrande| self.delta_grande(&igrande, 2.0))
            .sum();
        -asd
    }
}
impl std::fmt::Display for OArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for i in 0..self.N {
            for j in 0..self.k {
                write!(f, "{}", if self.d[j * self.N + i] { "1 " } else { "0 " });
            }
            writeln!(f, "");
        }
        writeln!(f, "Fittness: {}", self.fitness())
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let units: Vec<OArray> = (0..1000)
        .map(|_i| OArray::new_random_balanced(k, N, &mut rng))
        .collect();

    let f = Population::new(units)
        .set_size(1000)
        .set_breed_factor(0.3)
        .set_survival_factor(0.6)
        .epochs_parallel(3000, 4) // 4 CPU cores
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    println!("{}", asd.unwrap());
}

#[test]
fn new_random() {
    let a = OArray::new_random_balanced(k, N, &mut rand::thread_rng());
    println!("{:?}", a);
}

#[test]
fn check_fitness() {
    let test = OArray {
        d: vec![false, false, true, true, false, true, false, true],
        N: 4,
        k: 2,
    };
    println!("Fitnesssss: {}", test.fitness());
}
