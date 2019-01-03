use oarray::FitnessFunction::*;
use oarray::OArray;
use rand::thread_rng;
use rand::Rng;
use spiril::unit::Unit;
use std::f64;

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
            Walsh(x) => self.partial.walsh_incremental(x, &self.last_col),
            WalshFaster(x) => self.partial.walsh_incremental_faster(x, &self.last_col),
            DeltaFast => self.complete_oa().delta_incremental_faster(),
            _ => self.complete_oa().fitness(),
        }
    }
}
