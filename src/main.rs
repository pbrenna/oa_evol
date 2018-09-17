// This example implements the queens problem:
// https://en.wikipedia.org/wiki/Eight_queens_puzzle
// using an evolutionary algorithm.
#![feature(nll)]
extern crate ctrlc;
extern crate pbr;
extern crate rand;

// internal crates

extern crate spiril;

use rand::Rng;
use std::iter;

use spiril::{population::Population, unit::Unit};
mod t_combinations;

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
struct OArray {
    d: Vec<bool>,
    ngrande: usize,
    k: usize,
    target_t: usize,
}

//Parametri di esecuzione
const K: usize = 8;
const N: usize = 16;
const T: usize = 2;

/// Unisce due OArray in in modo che il risultato sia bilanciato
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
    /// Crea un array di larghezza `my_k` * `ngrande`,
    /// che si vorrà portare ad avere forza `t`, e lo inizializza
    /// in modo randomico ma bilanciato utilizzando `rng`.
    fn new_random_balanced(
        my_k: usize,
        ngrande: usize,
        target_t: usize,
        rng: &mut impl rand::Rng,
    ) -> Self {
        assert!(
            ngrande / (2usize.pow(target_t as u32)) >= 1,
            "I parametri N={},s=2,t={} non soddisfano i requisiti base per un array ortogonale",
            ngrande,
            target_t
        );
        let mut out = OArray {
            d: iter::repeat(&[true, false])
                .map(|i| *rng.choose(i).unwrap())
                .take(ngrande * my_k)
                .collect(),
            ngrande: ngrande,
            k: my_k,
            target_t: target_t,
        };
        for x in 0..K {
            rng.shuffle(&mut out.d[(x * ngrande)..(x * ngrande + ngrande)]);
        }
        out
    }

    /// conta il numero di occorrenze di `needle` nelle colonne `igrande` dell'array,
    /// e restituisce la differenza rispetto al livello `lambda`
    fn delta(&self, igrande: &[usize], needle: usize, lambda: usize) -> usize {
        let mut out = 0;
        for i in 0..self.ngrande {
            //iterate rows
            let cur_row = igrande.iter().rev().fold(0, |acc, col| {
                acc << 1 | (self.d[col * self.ngrande + i] as usize)
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
        let max_binary = 2usize.pow(t_num as u32) - 1;
        let lambda = self.ngrande / max_binary;
        let result = (0..max_binary)
            .map(|i| {
                let d = self.delta(igrande, i, lambda);
                (d as f64).powf(p)
            }).sum::<f64>()
            .powf(1.0 / p);
        result
    }

    /// Scambia due coordinate nel vettore con probabiltà `prob`,
    /// usando `rng`.
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let rand = rng.gen_range::<f64>(0.0, 1.0);
        if rand < prob {
            //pick random coordinate to mutate
            let coord1 = rng.gen_range(0, self.ngrande * self.k);
            //pick other coordinate to swap
            let mut coord2 = rng.gen_range(0, self.ngrande * self.k);
            while self.d[coord2] == self.d[coord1] {
                coord2 = rng.gen_range(0, self.ngrande * self.k);
            }
            self.d.swap(coord1, coord2);
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
            d: iter::repeat(false).take(self.ngrande * self.k).collect(),
            ngrande: self.ngrande,
            k: self.k,
            target_t: self.target_t,
        };
        for i in 0..K {
            let ngrande = self.ngrande;
            let col1 = &self.d[i * ngrande..i * ngrande + ngrande];
            let col2 = &other.d[i * ngrande..i * ngrande + ngrande];
            let col3 = &mut out.d[i * ngrande..i * ngrande + ngrande];
            balanced_crossover(col1, col2, col3, &mut rng);
        }
        out.mutate_with_prob(0.25, &mut rng);
        out
    }

    /// Fitness: calcola delta_grande per ogni combinazione di colonne e somma
    fn fitness(&self) -> f64 {
        let asd: f64 = t_combinations::Combinations::new(self.k, self.target_t)
            .iter()
            .map(|igrande| self.delta_grande(&igrande, 2.0))
            .sum();
        -asd
    }
}
impl std::fmt::Display for OArray {
    /// Stampa un OA
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for i in 0..self.ngrande {
            for j in 0..self.k {
                write!(
                    f,
                    "{}",
                    if self.d[j * self.ngrande + i] {
                        "1 "
                    } else {
                        "0 "
                    }
                );
            }
            writeln!(f, "");
        }
        writeln!(f, "Fittness: {}", self.fitness())
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let units: Vec<OArray> = (0..2000)
        .map(|_i| OArray::new_random_balanced(K, N, T, &mut rng))
        .collect();

    let epochs = 3000;
    let mut bar = pbr::ProgressBar::new(epochs);
    let (tx, rx) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    }).expect("Can't register ctrl+c");
    let f = Population::new(units)
        .set_size(2000)
        .set_breed_factor(0.4)
        .set_survival_factor(0.8)
        .register_callback(Box::new(move |i, j, count| {
            bar.message(&format!(
                " Best: {:.4}, Mean: {:.4}, Count: {}; iteration ",
                i, j, count
            ));
            (&mut bar).inc();
            if let Ok(_) = rx.try_recv() {
                false
            } else {
                true
            }
        })).epochs_parallel(epochs as u32, 4) // 4 CPU cores
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    println!("{}", asd.unwrap());
}

#[test]
fn new_random() {
    let a = OArray::new_random_balanced(K, N, T, &mut rand::thread_rng());
    println!("{:?}", a);
}

#[test]
fn check_fitness1() {
    let test = OArray {
        d: vec![false, false, true, true, false, true, false, true],
        ngrande: 4,
        k: 2,
        target_t: 2,
    };
    assert!(test.fitness() == 0.0);
}

#[test]
fn check_fitness2() {
    let test = OArray {
        d: vec![false, true, true, true, false, true, false, true],
        ngrande: 4,
        k: 2,
        target_t: 1,
    };
    assert!(test.fitness() != 0.0);
}
