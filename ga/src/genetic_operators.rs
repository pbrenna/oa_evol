use std::iter;
use oarray::OArray;
use rand::Rng;
use spiril::unit::Unit;
use rand::thread_rng;

/// Istanza di OArray dedicata all'algoritmo genetico: implementa 
/// la mutazione.
#[derive(Clone)]
pub struct GAOArray(pub OArray);

impl GAOArray{
    /// Scambia due coordinate nel vettore con probabiltà `prob`,
    /// usando `rng`.
    fn mutate_with_prob(&mut self, prob: f64, rng: &mut impl Rng) {
        let n = self.0.ngrande;
        for col in self.0.iter_cols_mut() {
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
    for j in pos {
        let mut cnt = [0, 0];
        let choice = if cnt[0] >= balance {
            true
        } else if cnt[1] >= balance {
            false
        } else {
            *r.choose(&[a[j], b[j]]).unwrap()
        };

        out[j] = choice;
        cnt[choice as usize] += 1;
    }
}

// implement trait functions mutate and calculate_fitness:
impl Unit for GAOArray {
    fn breed_with(&self, other: &Self) -> Self {
        let oa = &self.0;
        //println!("BREED++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        let mut rng = thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on each bitstring
        let mut out_inner = OArray::new(
            oa.ngrande,
            oa.k,
            oa.target_t,
            iter::repeat(false)
                .take(oa.ngrande * oa.k)
                .collect()
        );
        for (col1, (col2, col3)) in oa
            .iter_cols()
            .zip(other.0.iter_cols().zip(out_inner.iter_cols_mut()))
        {
            balanced_crossover(col1, col2, col3, &mut rng);
        }
        let mut out = GAOArray(out_inner);
        out.mutate_with_prob(0.2, &mut rng);
        out
    }

    /// Fitness: calcola delta_grande per ogni combinazione di colonne e somma
    fn fitness(&self) -> f64 {
        self.0.delta_fitness()
    }
}
#[test]
fn mutation() {
    let mut r = rand::thread_rng();
    let mut a = GAOArray(OArray::new_random_balanced(8, 4, 3, &mut r));
    let b = a.clone();
    assert!(a.0.d == b.0.d);
    a.mutate_with_prob(1.0, &mut r);
    assert!(a.0.d != b.0.d);
    let c = a.breed_with(&b);
    assert!(a.0.d != c.0.d);
    assert!(b.0.d != c.0.d);
}