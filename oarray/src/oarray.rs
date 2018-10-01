use num_iter::range;
use t_combinations::Combinations;
use rand::Rng;
use std::fmt::{Display, Formatter, Error};
use std::f64::EPSILON;

use alphabet::Alphabet;

#[derive(Clone, Debug)]
/// Array ortogonale di dimensione ngrande * k, che si vuole portare a forza t.
pub struct OArray<T: Alphabet> {
    pub ngrande: usize,
    pub k: usize,
    pub s: T,
    pub target_t: usize,
    pub d: Vec<T>,
}




impl<T: Alphabet> OArray<T> {
    pub fn new(ngrande: usize, k: usize, s: T, target_t: usize, d: Vec<T>) -> Self{
        OArray {
            //ripete l'alfabeto ngrande*k volte
            d,
            ngrande,
            k,
            s,
            target_t,
        }
    }

    /// Crea un array di larghezza `k` * `ngrande`,
    /// che si vorrà portare ad avere forza `t`, e lo inizializza
    /// in modo randomico ma bilanciato utilizzando `rng`.
    pub fn new_random_balanced(
        ngrande: usize,
        k: usize,
        s: T,
        target_t: usize,
        rng: &mut impl Rng,
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
    pub fn fitness(&self) -> f64 {
        let asd: f64 = Combinations::new(self.k, self.target_t)
            .iter()
            .map(|igrande| self.delta_grande(&igrande, 2.0))
            .sum();
        -asd
    }
}
impl<T: Alphabet> Display for OArray<T> {
    /// Stampa un OA
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let fit = self.fitness();
        if -fit < EPSILON {
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
