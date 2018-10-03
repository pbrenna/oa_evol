use rand::Rng;
use std::f64::EPSILON;
use std::fmt::{Display, Error, Formatter};
use t_combinations::Combinations;
use streaming_iterator::StreamingIterator;

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
    pub fn fitness(&self) -> f64 {
        let mut comb = Combinations::new(self.k, self.target_t);
        let asd : f64 = comb.stream_iter().map(|igrande| self.delta_grande(&igrande, 2.0)).cloned()
            .sum();
        -asd
    }
}
impl Display for OArray {
    /// Stampa un OA
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let fit = self.fitness();
        if -fit < EPSILON {
            writeln!(
                f,
                "OA[N: {ngrande}, k: {k}, s: 2, t: {t}], ({ngrande}, {k}, {t}, {lambda})",
                ngrande = self.ngrande,
                k = self.k,
                t = self.target_t,
                lambda = self.lambda
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
    let a = OArray::new_random_balanced(8, 4, 3, &mut rand::thread_rng());
    for col in a.iter_cols() {
        let num0 = col.iter().filter(|&&i| i).count();
        let num1 = col.iter().filter(|&&i| !i).count();
        assert!(num0 == num1);
    }
}

#[test]
fn check_fitness1() {
    let test = OArray::new(4, 2, 2, bool_vec![0, 0, 1, 1, 0, 1, 0, 1]);
    assert!(test.fitness() == 0.0);
}

#[test]
fn check_fitness2() {
    let test = OArray::new(4, 2, 1, bool_vec![0, 1, 1, 1, 0, 1, 0, 1]);
    assert!(test.fitness() != 0.0);
}
