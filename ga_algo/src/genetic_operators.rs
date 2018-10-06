use oarray::OArray;
use rand::thread_rng;
use rand::Rng;
use spiril::unit::Unit;
use std::iter;

/// Istanza di OArray dedicata all'algoritmo genetico: implementa
/// la mutazione.
#[derive(Clone)]
pub struct GAOArray {
    pub oa: OArray,
    pub mutation_prob: f64,
}

impl GAOArray {
    /// Scambia due coordinate nel vettore con probabiltà `prob`,
    /// usando `rng`.
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let n = self.oa.ngrande;
        for col in self.oa.iter_cols_mut() {
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
impl Unit for GAOArray {
    fn breed_with(&self, other: &Self) -> Self {
        let oa = &self.oa;
        //println!("BREED++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        let mut rng = thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on each bitstring
        let mut out_inner = OArray::new(
            oa.ngrande,
            oa.k,
            oa.target_t,
            iter::repeat(false).take(oa.ngrande * oa.k).collect(),
        );
        for (col1, (col2, col3)) in oa
            .iter_cols()
            .zip(other.oa.iter_cols().zip(out_inner.iter_cols_mut()))
        {
            balanced_crossover(col1, col2, col3, &mut rng);
        }
        let mut out = GAOArray {
            oa: out_inner,
            mutation_prob: self.mutation_prob,
        };
        out.mutate_with_prob(self.mutation_prob, &mut rng);
        out
    }

    /// Fitness: calcola delta_grande per ogni combinazione di colonne e somma
    fn fitness(&self) -> f64 {
        self.oa.delta_fitness()
    }
}
#[test]
fn mutation() {
    let mut r = thread_rng();
    let mut a = GAOArray {
        oa: OArray::new_random_balanced(8, 4, 3, &mut r),
        mutation_prob: 0.5,
    };
    let b = a.clone();
    assert!(a.oa.d == b.oa.d);
    a.mutate_with_prob(1.0, &mut r);
    assert!(a.oa.d != b.oa.d);
    let c = a.breed_with(&b);
    assert!(a.oa.d != c.oa.d);
    assert!(b.oa.d != c.oa.d);
}

#[test]
fn balanced_crossover_test() {
    let mut r = thread_rng();
    for _ in 0..100 {
        let mut a = GAOArray {
            oa: OArray::new_random_balanced(8, 1, 1, &mut r),
            mutation_prob: 0.5,
        };
        let mut b = GAOArray {
            oa: OArray::new_random_balanced(8, 1, 1, &mut r),
            mutation_prob: 0.5,
        };
        let mut c = GAOArray {
            oa: OArray::new_random_balanced(8, 1, 1, &mut r),
            mutation_prob: 0.5,
        };
        assert!(is_balanced(&a.oa.d));
        assert!(is_balanced(&b.oa.d));
        balanced_crossover(&a.oa.d, &b.oa.d, &mut c.oa.d, &mut r);
        assert!(is_balanced(&c.oa.d));
    }
}

#[allow(unused)]
fn is_balanced(v: &[bool]) -> bool {
    let n_ones = v.iter().filter(|&&i| i).count();
    let n_zeros = v.iter().filter(|&&i| !i).count();
    n_ones == n_zeros
}
