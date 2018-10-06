#[allow(unused_imports)]
use rand::{Rng, thread_rng};
use std::f64::EPSILON;
use std::fmt::{Display, Error, Formatter};
use streaming_iterator::StreamingIterator;
use t_combinations::Combinations;

#[derive(Clone, Debug)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
pub struct OArray {
    pub ngrande: usize,
    pub k: usize,
    pub target_t: u32,
    lambda: usize,
    pub d: Vec<bool>,
}

impl OArray {
    pub fn new(ngrande: usize, k: usize, target_t: u32, d: Vec<bool>) -> Self {
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
    ) -> Self {
        //ripete l'alfabeto ngrande*k volte
        let data = [true, false]
            .iter()
            .cloned()
            .cycle()
            .take(ngrande * k)
            .collect();
        let mut out = OArray::new(ngrande, k, target_t, data);
        //mescola ogni colonna
        for x in out.iter_cols_mut() {
            rng.shuffle(x);
        }
        out
    }

    /// conta il numero di occorrenze di `needle` nelle colonne `igrande` dell'array,
    /// e restituisce la differenza rispetto al livello `lambda`
    #[allow(unused)]
    fn delta(&self, igrande: &[usize], needle: usize, lambda: usize) -> usize {
        let mut out = 0;
        for i in 0..self.ngrande {
            //iterate rows
            let cur_row = igrande.iter().fold(0, |acc, col| {
                (acc << 1) | (self.d[col * self.ngrande + i] as usize)
            });
            if cur_row == needle {
                out += 1
            }
        }
        (lambda as isize - out as isize).abs() as usize
    }

    /// calcola per ogni numero rappresentabile da `igrande.len` bit
    /// la funzione delta, usa i risultati per dare una distanza.
    #[allow(unused)]
    fn delta_grande(&self, igrande: &[usize], p: f64) -> f64 {
        let t_num = igrande.len();
        let num_representable_strings = 2usize.pow(t_num as u32);
        let lambda = self.ngrande / num_representable_strings;
        (0..num_representable_strings) //last is excluded
            .map(|i| {
                let d = self.delta(igrande, i, lambda);
                (d as f64).powf(p)
            })
            .sum::<f64>()
            .powf(1.0 / p)
    }

    fn delta_grande_faster(&self, igrande: &[usize], p: u32) -> f64 {
        let t_num = igrande.len();
        let num_representable_strings = 2usize.pow(t_num as u32);
        let lambda = self.ngrande / num_representable_strings;
        let mut counts = vec![lambda as i64; num_representable_strings];
        for i in 0..self.ngrande {
            let cur_row = igrande.iter().fold(0, |acc, col| {
                acc * 2 + (self.d[col * self.ngrande + i] as usize)
            });
            counts[cur_row] -= 1;
        }
        let tot: i64 = counts.iter().map(|&i| i.abs().pow(p)).sum();
        (tot as f64).powf(1.0 / f64::from(p))
    }
    /// Walsh
    pub fn walsh_fitness(&self) -> f64 {
        let t = self.target_t;
        let mut grand_tot = 0;
        for w in 1..=t {
            let mut combs = Combinations::new(self.k, w);
            let mut comb_iter = combs.stream_iter();
            while let Some(comb) = comb_iter.next() {
                let mut vec_tot = 0i64;
                for u in self.iter_rows() {
                    let prod = comb.iter().map(|i| u[*i]).fold(false, |acc, cur| acc ^ cur);
                    vec_tot += if prod { 1 } else { -1 };
                }
                grand_tot += vec_tot.abs();
            }
        }
        -grand_tot as f64
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
    #[allow(unused)]
    fn fitness_old_old(&self) -> f64 {
        let mut comb = Combinations::new(self.k, self.target_t);
        let asd: f64 = comb
            .stream_iter()
            .map(|igrande| self.delta_grande(&igrande, 2.0))
            .cloned()
            .sum();
        -asd
    }
    pub fn delta_fitness(&self) -> f64 {
        let mut comb = Combinations::new(self.k, self.target_t);
        let asd: f64 = comb
            .stream_iter()
            .map(|igrande| self.delta_grande_faster(&igrande, 2))
            .cloned()
            .sum();
        -asd
    }
}

impl Display for OArray {
    /// Stampa un OA
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let d_fit = self.delta_fitness();
        if -d_fit < EPSILON {
            writeln!(
                f,
                "OA[N: {ngrande}, k: {k}, s: 2, t: {t}], ({ngrande}, {k}, {t}, {lambda}); delta_fitness: {fit}, walsh: {walsh}",
                ngrande = self.ngrande,
                k = self.k,
                t = self.target_t,
                lambda = self.lambda,
                fit=d_fit,
                walsh=self.walsh_fitness()
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

#[allow(unused_macros)]
macro_rules! bool_vec {
    ($($x:expr),*) => {
        vec![$($x != 0),*]
    };
}
#[test]
fn new_random() {
    let a = OArray::new_random_balanced(8, 4, 3, &mut thread_rng());
    for col in a.iter_cols() {
        let num0 = col.iter().filter(|&&i| i).count();
        let num1 = col.iter().filter(|&&i| !i).count();
        assert!(num0 == num1);
    }
}

#[test]
fn check_fitness1() {
    let test = OArray::new(4, 2, 2, bool_vec![0, 0, 1, 1, 0, 1, 0, 1]);
    assert!(test.delta_fitness() == 0.0);
    assert!(test.walsh_fitness() == 0.0);
}

#[test]
fn check_fast_delta() {
    let mut rng = thread_rng();
    let error = EPSILON;
    for _ in 0..100 {
        let rand = OArray::new_random_balanced(8, 7, 3, &mut rng);
        assert!((rand.fitness_old_old() - rand.delta_fitness()).abs() < error);
    }
}

#[test]
fn check_fitness2() {
    let test = OArray::new(4, 2, 1, bool_vec![0, 1, 1, 1, 0, 1, 0, 1]);
    assert!(test.delta_fitness() != 0.0);
    assert!(test.walsh_fitness() != 0.0);
}
