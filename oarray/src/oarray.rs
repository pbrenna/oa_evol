#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::f64::EPSILON;
use std::fmt::{Display, Error, Formatter};

use fitness::FitnessFunction;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
pub struct OArray {
    pub ngrande: usize,
    pub k: usize,
    pub target_t: u32,
    lambda: usize,
    pub d: Vec<bool>,
    pub fitness_f: FitnessFunction,
}

impl OArray {
    pub fn new(
        ngrande: usize,
        k: usize,
        target_t: u32,
        d: Vec<bool>,
        fitness_f: FitnessFunction,
    ) -> Self {
        let num_t_strings = 2usize.pow(target_t);
        let lambda = ngrande / num_t_strings;
        assert!(
            lambda >= 1 && num_t_strings * lambda == ngrande,
            "I parametri N={},s=2,t={} non soddisfano i requisiti base per un array ortogonale",
            ngrande,
            target_t
        );
        OArray {
            //ripete l'alfabeto ngrande*k volte
            ngrande,
            k,
            target_t,
            lambda,
            d,
            fitness_f,
        }
    }

    /// Crea un array di larghezza `k` * `ngrande`,
    /// che si vorrà portare ad avere forza `t`, e lo inizializza
    /// in modo randomico ma bilanciato utilizzando `rng`.
    pub fn new_random_balanced(
        ngrande: usize,
        k: usize,
        target_t: u32,
        rng: &mut impl Rng,
        fitness_f: FitnessFunction,
    ) -> Self {
        //ripete l'alfabeto ngrande*k volte
        let data = [true, false]
            .iter()
            .cloned()
            .cycle()
            .take(ngrande * k)
            .collect();
        let mut out = OArray::new(ngrande, k, target_t, data, fitness_f);
        //mescola ogni colonna
        for x in out.iter_cols_mut() {
            rng.shuffle(x);
        }
        out
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = &[bool]> {
        self.d.chunks(self.ngrande)
    }
    pub fn iter_cols_mut(&mut self) -> impl Iterator<Item = &mut [bool]> {
        self.d.chunks_mut(self.ngrande)
    }
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<&bool>> {
        let b = self.ngrande;
        (0..b).map(move |i| (&self.d[i..]).iter().step_by(b).collect())
    }

    /*pub fn check_linear_cols(&self) -> bool {
        let cols: Vec<&[bool]> = self.iter_cols().collect();
        for a in 0..self.k {
            for b in a + 1..self.k {
                let xor: Vec<bool> = cols[a]
                    .iter()
                    .zip(cols[b].iter())
                    .map(|(&a, &b)| a ^ b)
                    .collect();
                if !cols.iter().any(|&row| row == xor.as_slice()) {
                    return false;
                }
            }
        }
        true
    }*/
    pub fn check_linear(&self) -> bool {
        let rows: Vec<Vec<&bool>> = self.iter_rows().collect();
        let has_row = |xor| {
            rows.iter().any(|row| {
                let v: Vec<bool> = row.iter().map(|&i| *i).collect();
                v == xor
            })
        };
        if !has_row(vec![false; self.k]) {
            return false;
        }
        for a in 0..self.ngrande {
            for b in a + 1..self.ngrande {
                let xor: Vec<bool> = rows[a]
                    .iter()
                    .zip(rows[b].iter())
                    .map(|(&a, &b)| a ^ b)
                    .collect();
                if !has_row(xor) {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for OArray {
    /// Stampa un OA
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let d_fit = self.fitness();
        if -d_fit < EPSILON {
            writeln!(
                f,
                "OA[N: {ngrande}, k: {k}, s: 2, t: {t}], ({ngrande}, {k}, {t}, {lambda}); fitness: {fit}, fitness_f: {fitness_f:?}, linear: {lin}",
                ngrande = self.ngrande,
                k = self.k,
                t = self.target_t,
                lambda = self.lambda,
                fit=d_fit,
                fitness_f=self.fitness_f,
                lin=self.check_linear(),
            )?;
        }
        for row in self.iter_rows() {
            for x in row {
                let x_conv = *x as usize;
                write!(f, "{} ", x_conv)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[test]
fn new_random() {
    let a = OArray::new_random_balanced(8, 4, 3, &mut thread_rng(), FitnessFunction::DeltaFast);
    for col in a.iter_cols() {
        let num0 = col.iter().filter(|&&i| i).count();
        let num1 = col.iter().filter(|&&i| !i).count();
        assert!(num0 == num1);
    }
}
