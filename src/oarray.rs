use num_iter::range;
use rand::Rng;
use std::iter;

use spiril::unit::Unit;

use alphabet::Alphabet;
use t_combinations::Combinations;

#[derive(Clone, Debug)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
pub struct OArray<T: Alphabet> {
    ngrande: usize,
    k: usize,
    s: T,
    target_t: usize,
    d: Vec<T>,
}


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
    pub fn new_random_balanced(
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
        let base = self.s_usize();
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

    fn s_usize(&self) -> usize {
        self.s.to_usize().unwrap()
    }

    /// calcola per ogni numero rappresentabile da `igrande.len` bit
    /// la funzione delta, usa i risultati per dare una distanza.
    fn delta_grande(&self, igrande: &[usize], p: f64) -> f64 {
        let t_num = igrande.len();
        let num_representable_strings = self.s_usize().pow(t_num as u32);
        let lambda = self.ngrande / num_representable_strings;
        (0..num_representable_strings) //last is excluded 
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

    pub fn iter_cols(&self) -> impl Iterator<Item = &[T]> {
        self.d.chunks(self.ngrande)
    }
    pub fn iter_cols_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.d.chunks_mut(self.ngrande)
    }
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
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
        let asd: f64 = Combinations::new(self.k, self.target_t)
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
                self.ngrande, self.k, self.s_usize(), self.target_t
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

#[test]
fn mutation() {
    let mut r = rand::thread_rng();
    let mut a = OArray::new_random_balanced(8, 4, 2u8, 3, &mut r);
    let b = a.clone();
    assert!(a.d == b.d);
    a.mutate_with_prob(1.0, &mut r);
    assert!(a.d != b.d);
    let c = a.breed_with(&b);
    assert!(a.d != c.d);
    assert!(b.d != c.d);
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


#[test]
fn new_random() {
    let a = OArray::new_random_balanced(8, 4, 2u8, 3, &mut rand::thread_rng());
    for col in a.iter_cols() {
        let num0 = col.iter().filter(|&&i| i == 1).count();
        let num1 = col.iter().filter(|&&i| i == 0).count();
        assert!(num0 == num1);
    }
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
