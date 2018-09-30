use oarray::alphabet::Alphabet;
use std::iter;
use oarray::OArray;
use num_iter::range;
use rand::Rng;
use spiril::unit::Unit;

/// Istanza di OArray dedicata all'algoritmo genetico: implementa 
/// la mutazione.
#[derive(Clone)]
pub struct GAOArray<T: Alphabet>(pub OArray<T>);

impl<T: Alphabet> GAOArray<T>{
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

// implement trait functions mutate and calculate_fitness:
impl<T: Alphabet> Unit for GAOArray<T> {
    fn breed_with(&self, other: &Self) -> Self {
        let oa = &self.0;
        //println!("BREED++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        let mut rng = rand::thread_rng();
        //GA crossover and mutation operators are applied
        //component-wise on each bitstring
        let mut out_inner: OArray<T> = OArray {
            d: iter::repeat(T::zero())
                .take(oa.ngrande * oa.k)
                .collect(),
            ngrande: oa.ngrande,
            k: oa.k,
            s: oa.s,
            target_t: oa.target_t,
        };
        for (col1, (col2, col3)) in oa
            .iter_cols()
            .zip(other.0.iter_cols().zip(out_inner.iter_cols_mut()))
        {
            balanced_crossover(col1, col2, col3, oa.s, &mut rng);
        }
        let mut out = GAOArray(out_inner);
        out.mutate_with_prob(0.2, &mut rng);
        out
    }

    /// Fitness: calcola delta_grande per ogni combinazione di colonne e somma
    fn fitness(&self) -> f64 {
        self.0.fitness()
    }
}
#[test]
fn mutation() {
    let mut r = rand::thread_rng();
    let mut a = GAOArray(OArray::new_random_balanced(8, 4, 2u8, 3, &mut r));
    let b = a.clone();
    assert!(a.0.d == b.0.d);
    a.mutate_with_prob(1.0, &mut r);
    assert!(a.0.d != b.0.d);
    let c = a.breed_with(&b);
    assert!(a.0.d != c.0.d);
    assert!(b.0.d != c.0.d);
}