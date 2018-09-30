extern crate ctrlc;
extern crate num;
extern crate num_iter;
extern crate num_traits;
extern crate pbr;
extern crate rand;

// internal crates

extern crate spiril;

use rand::Rng;
use std::iter;

use num::Integer;
use num_iter::range;
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use spiril::{population::Population, unit::Unit};
mod epoch;
mod t_combinations;
use epoch::TournamentEpoch;
use std::ops::Add;

trait Alphabet:
    Integer + Clone + Add<Self, Output = Self> + One + Zero + FromPrimitive + ToPrimitive + Send + Copy
{
}
impl<
        T: Integer
            + Clone
            + Add<Self, Output = Self>
            + One
            + Zero
            + FromPrimitive
            + ToPrimitive
            + Send
            + Copy,
    > Alphabet for T
{}

#[derive(Clone, Debug)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
struct OArray<T: Alphabet> {
    ngrande: usize,
    k: usize,
    s: T,
    target_t: usize,
    d: Vec<T>,
}

//Parametri di esecuzione
const N: usize = 9;
const K: usize = 4;
const S: u8 = 3;
const T: usize = 2;

/// Unisce due OArray in in modo che il risultato sia bilanciato
fn balanced_crossover<T: Alphabet>(a: &[T], b: &[T], out: &mut [T], s: T, r: &mut impl Rng) {
    let ngrande = a.len();
    let s_size = s.to_usize().unwrap();
    let balance = ngrande / s_size;
    assert!(balance * s_size == ngrande);
    let mut cnt = vec![balance; s_size];
    let mut pos: Vec<_> = (0..ngrande).collect();
    r.shuffle(&mut pos);
    for j in pos {
        let a_usize = a[j].to_usize().unwrap();
        let b_usize = b[j].to_usize().unwrap();
        let choice;
        //If choosing neither a or b affects balancedness..
        if cnt[a_usize] > 0 && cnt[b_usize] > 0 {
            //choose randomly
            choice = *r.choose(&[a[j], b[j]]).unwrap();
        } else if cnt[a_usize] == 0 && cnt[b_usize] > 0 {
            //only b can be choosen
            choice = b[j];
        } else if cnt[a_usize] > 0 && cnt[b_usize] == 0 {
            //only a can be choosen
            choice = a[j];
        } else {
            //choose randomly amongst all symbols that don't affect balancedness
            let possible_choices: Vec<T> = range(T::zero(), s)
                .enumerate()
                .filter(|&(i, _)| cnt[i] > 0)
                .map(|(_, j)| j)
                .collect();
            choice = *r.choose(&possible_choices).unwrap();
        }
        assert!(choice < s);

        out[j] = choice;
        let choice_u = choice.to_usize().unwrap();
        cnt[choice_u] -= 1;
    }
}

impl<T: Alphabet> OArray<T> {
    /// Crea un array di larghezza `k` * `ngrande`,
    /// che si vorrà portare ad avere forza `t`, e lo inizializza
    /// in modo randomico ma bilanciato utilizzando `rng`.
    fn new_random_balanced(
        ngrande: usize,
        k: usize,
        s: T,
        target_t: usize,
        rng: &mut impl rand::Rng,
    ) -> Self {
        let s_usize = s.to_usize().unwrap();
        assert!(
            ngrande / (s_usize.pow(target_t as u32)) >= 1,
            "I parametri N={},s={},t={} non soddisfano i requisiti base per un array ortogonale",
            ngrande,
            s_usize,
            target_t
        );
        let mut out: OArray<T> = OArray {
            //ripete l'alfabeto ngrande*k volte
            d: range(T::zero(), s).cycle().take(ngrande * k).collect(),
            ngrande,
            k,
            s,
            target_t,
        };
        //mescola ogni colonna
        for x in out.iter_cols_mut() {
            rng.shuffle(x);
        }
        out
    }

    /// conta il numero di occorrenze di `needle` nelle colonne `igrande` dell'array,
    /// e restituisce la differenza rispetto al livello `lambda`
    fn delta(&self, igrande: &[usize], needle: usize, lambda: usize) -> usize {
        let mut out = 0;
        let base = self.s.to_usize().unwrap();
        for i in 0..self.ngrande {
            //iterate rows
            let cur_row = igrande.iter().fold(0, |acc, col| {
                acc * base + self.d[col * self.ngrande + i].to_usize().unwrap()
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
        let max_representable = self.s.to_usize().unwrap().pow(t_num as u32) - 1;
        let lambda = self.ngrande / max_representable;
        (0..max_representable)
            .map(|i| {
                let d = self.delta(igrande, i, lambda);
                (d as f64).powf(p)
            }).sum::<f64>()
            .powf(1.0 / p)
    }

    /// Scambia due coordinate nel vettore con probabiltà `prob`,
    /// usando `rng`.
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let n = self.ngrande;
        for col in self.iter_cols_mut() {
            if rng.gen_range::<f64>(0.0, 1.0) < prob {
                //pick random coordinate to mutate
                let coord1 = rng.gen_range(0, n);
                //pick other coordinate to swap
                let mut coord2 = rng.gen_range(0, n);
                while col[coord2] == col[coord1] {
                    coord2 = rng.gen_range(0, n);
                }
                col.swap(coord1, coord2);
            }
        }
    }

    fn iter_cols(&self) -> impl Iterator<Item = &[T]> {
        self.d.chunks(self.ngrande)
    }
    fn iter_cols_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.d.chunks_mut(self.ngrande)
    }
    fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
        let b = self.ngrande;
        (0..b).map(move |i| (&self.d[i..]).iter().step_by(b).collect())
    }
}
// implement trait functions mutate and calculate_fitness:
impl<T: Alphabet> Unit for OArray<T> {
    fn breed_with(&self, other: &Self) -> Self {
        //println!("BREED++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        let mut rng = rand::thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on each bitstring
        let mut out: OArray<T> = OArray {
            d: iter::repeat(T::zero())
                .take(self.ngrande * self.k)
                .collect(),
            ngrande: self.ngrande,
            k: self.k,
            s: self.s,
            target_t: self.target_t,
        };
        for (col1, (col2, col3)) in self
            .iter_cols()
            .zip(other.iter_cols().zip(out.iter_cols_mut()))
        {
            balanced_crossover(col1, col2, col3, self.s, &mut rng);
        }
        out.mutate_with_prob(0.2, &mut rng);
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
impl<T: Alphabet> std::fmt::Display for OArray<T> {
    /// Stampa un OA
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let fit = self.fitness();
        if -fit < std::f64::EPSILON {
            writeln!(
                f,
                "OA(N: {}, k: {}, s: {}, t: {})",
                self.ngrande, self.k, self.s.to_usize().unwrap(), self.target_t
            )?;
        }
        for row in self.iter_rows() {
            for x in row {
                let x_conv = x.to_usize().unwrap();
                write!(f, "{} ", x_conv)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn main() {
    let n_units = 50;
    let epochs = 10000;
    let mut rng = rand::thread_rng();

    let units: Vec<OArray<_>> = (0..n_units)
        .map(|_i| OArray::new_random_balanced(N, K, S, T, &mut rng))
        .collect();

    let mut pbar = pbr::ProgressBar::new(epochs);
    let (tx, rx) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    }).expect("Can't register ctrl+c");

    let epoch = TournamentEpoch::new(500_000);
    //let epoch = spiril::epoch::DefaultEpoch::new(0.2, 0.8);
    let f = Population::new(units)
        .set_size(n_units)
        .set_breed_factor(0.2)
        .set_survival_factor(0.8)
        .register_callback(Box::new(move |i, j| {
            pbar.message(&format!(" Best: {:.4}, Mean: {:.4}; iteration ", i, j));
            /*for x in units {
                println!("{}", x.unit);
            }*/
            (&mut pbar).inc();
            rx.try_recv().is_err()
        })).epochs_parallel(epochs as u32, 4, &epoch) // 4 CPU cores
        .finish();
    let asd = f
        .iter()
        .max_by(|&a, &b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    if -asd.unwrap().fitness() < std::f64::EPSILON {
        println!("\n\n{}\n\n", asd.unwrap());
        std::process::exit(2);
    } else {
        std::process::exit(1);
    }
}

#[test]
fn new_random() {
    let a = OArray::new_random_balanced(N, K, 2u8, T, &mut rand::thread_rng());
    for col in a.iter_cols() {
        let num0 = col.iter().filter(|&&i| i == 1).count();
        let num1 = col.iter().filter(|&&i| i == 0).count();
        assert!(num0 == num1);
    }
}

#[test]
fn mutation() {
    let mut r = rand::thread_rng();
    let mut a = OArray::new_random_balanced(N, K, 2u8, T, &mut r);
    let b = a.clone();
    assert!(a.d == b.d);
    a.mutate_with_prob(1.0, &mut r);
    assert!(a.d != b.d);
    let c = a.breed_with(&b);
    assert!(a.d != c.d);
    assert!(b.d != c.d);
}

#[test]
fn check_fitness1() {
    let test = OArray {
        d: vec![0, 0, 1, 1, 0, 1, 0, 1],
        ngrande: 4,
        k: 2,
        s: 2u8,
        target_t: 2,
    };
    assert!(test.fitness() == 0.0);
    let test = OArray {
        d: vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2],
        ngrande: 9,
        k: 2,
        s: 3u8,
        target_t: 2,
    };
    assert!(test.fitness() == 0.0);
}

#[test]
fn check_fitness2() {
    let test = OArray {
        d: vec![0, 1, 1, 1, 0, 1, 0, 1],
        ngrande: 4,
        k: 2,
        s: 2u8,
        target_t: 1,
    };
    assert!(test.fitness() != 0.0);
}

#[test]
fn check_delta() {
    let test = OArray {
        ngrande: 3,
        k: 3,
        s: 3u8,
        target_t: 1,
        d: vec![2, 1, 0, 1, 1, 1, 0, 1, 3],
    };
    assert!(test.delta(&[0, 1], 0, 1) == 1);
    assert!(test.delta(&[0, 1], 1, 1) == 0);
    assert!(test.delta(&[0, 1], 2, 1) == 1);
    assert!(test.delta(&[0, 1], 3, 1) == 1);
    assert!(test.delta(&[0, 1], 4, 1) == 0);
    assert!(test.delta(&[0, 1], 5, 1) == 1);
    assert!(test.delta(&[0, 1], 6, 1) == 1);
    assert!(test.delta(&[0, 1], 7, 1) == 0);
    assert!(test.delta(&[0, 1], 8, 1) == 1);
    assert!(test.delta(&[0, 1, 2], 13, 1) == 0);
}
