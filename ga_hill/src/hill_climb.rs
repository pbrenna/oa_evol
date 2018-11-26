use genetic_operators::GAOArray;
use oarray::binary_strings;
use oarray::wtform::*;
use oarray::OArray;
use rand::{thread_rng, Rng};

impl GAOArray {
    pub fn hill_climb(&mut self, prob: f64) {
        if thread_rng().gen_range::<f64>(0.0, 1.0) >= prob {
            return;
        }
        assert!(self.oa.k < 64);
        let mut truth = self.oa.truth_table();
        let wtform = PolarTruthTable::from(&truth).walsh_tform();
        let cidev_t = wtform.cidev(self.oa.target_t as usize) as i32;

        //Calcolo gli insiemi dei vettori. Uso interi di 64 bit come vettori
        let mut w1p = vec![];
        let mut w1m = vec![];
        let mut w2p = vec![];
        let mut w2m = vec![];
        let mut w3p = vec![];
        let mut w3m = vec![];

        let a = cidev_t;
        let b = cidev_t - 2;
        let c = cidev_t - 4;

        for (index, val) in wtform.table.iter().enumerate() {
            let val = *val;
            let v = if val == a {
                Some(&mut w1p)
            } else if val == b {
                Some(&mut w2p)
            } else if val == c {
                Some(&mut w3p)
            } else if val == -a {
                Some(&mut w1m)
            } else if val == -b {
                Some(&mut w2m)
            } else if val == -c {
                Some(&mut w3m)
            } else {
                None
            };
            if let Some(vec) = v {
                if binary_strings::usize_hamming_weight(index) <= self.oa.target_t as usize {
                    vec.push(index);
                }
            }
        }
        let mut found = None;
        //this scope encloses the borrow of truth.table (lexical lifetime workaround)
        {
            //This lambda checks the conditions for (x1, x2) to be an improvement set
            let check_conds = |x1, x2| {
                //condizione 1
                truth.table[x1] != truth.table[x2]
                    && {
                        //condizione 2
                        w1p.iter()
                            .chain(w1m.iter())
                            .all(|&omega| scalar_prod(omega, x1) != scalar_prod(omega, x2))
                    }
                    && {
                        //condizione 3
                        w1p.iter().all(|&omega| {
                            truth.table[x1] == scalar_prod(omega, x1)
                                && truth.table[x2] == scalar_prod(omega, x2)
                        })
                    }
                    && {
                        //condizione 4
                        w1m.iter().all(|&omega| {
                            truth.table[x1] != scalar_prod(omega, x1)
                                && truth.table[x2] != scalar_prod(omega, x2)
                        })
                    }
                    && {
                        //condizione 5
                        w2p.iter().chain(w3p.iter()).all(|&omega| {
                            if scalar_prod(omega, x1) != scalar_prod(omega, x2) {
                                [x1, x2]
                                    .iter()
                                    .all(|&xi| truth.table[xi] == scalar_prod(omega, xi))
                            } else {
                                true
                            }
                        })
                    }
                    && {
                        //condizione 6
                        w2m.iter().chain(w3m.iter()).all(|&omega| {
                            if scalar_prod(omega, x1) != scalar_prod(omega, x2) {
                                [x1, x2]
                                    .iter()
                                    .all(|&xi| truth.table[xi] != scalar_prod(omega, xi))
                            } else {
                                true
                            }
                        })
                    }
            };

            let max_x = wtform.table.len();
            'outer: for i in 0..max_x {
                for j in i + 1..max_x {
                    if check_conds(i, j) {
                        //println!("Found improvement set: {},{}", i, j);
                        found = Some((i, j));
                        break 'outer;
                    }
                }
            }
        }
        if let Some((i, j)) = found {
            let old_f = self.oa.fitness();
            truth.table.swap(i, j);
            let old_oa = self.oa.clone();
            let new = OArray::from_truth_table(
                &truth,
                self.oa.ngrande,
                self.oa.target_t,
                self.oa.fitness_f,
            );
            if let Some(new_inner) = new {
                debug!("Sostituisco");
                self.oa = new_inner;
                let new_f = self.oa.fitness();
                let tform = PolarTruthTable::from(&truth).walsh_tform();
                let cidev_t_new = tform.cidev(self.oa.target_t as usize) as i32;
                debug_assert!(
                    new_f >= old_f,
                    "Old fitness:{}
                    New fitness:{}
                    Fitness function: {:?}
                    Old cidev: {},
                    New cidev: {},
                    Old OA: \n{}
                    New OA: \n{}",
                    old_f,
                    new_f,
                    self.oa.fitness_f,
                    cidev_t,
                    cidev_t_new,
                    old_oa,
                    self.oa
                );
            }
        }
    }
}
fn scalar_prod(x1: usize, x2: usize) -> bool {
    let mut prods = x1 & x2;
    let mut out = false;
    while prods != 0 {
        out ^= prods & 1 == 1;
        prods >>= 1;
    }
    out
}

#[test]
fn test_scalar_prod() {
    assert!(!scalar_prod(321, 552));
    assert!(scalar_prod(321, 553));
}
