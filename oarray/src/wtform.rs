use binary_strings::{hamming_weight, vec2usize, BinaryStringIterator};
use fitness::FitnessFunction;
use oarray::OArray;
use std::fmt::{Display, Error, Formatter};

pub struct TruthTable {
    pub table: Vec<bool>,
    pub log2len: usize,
}

pub struct PolarTruthTable {
    pub table: Vec<i32>,
    pub log2len: usize,
}

pub struct WalshTform {
    pub table: Vec<i32>,
    pub log2len: usize,
}

impl TruthTable {
    pub fn new(table: Vec<bool>) -> TruthTable {
        let l = (table.len() as f64).log2() as usize;
        TruthTable { table, log2len: l }
    }
}
impl<'a> From<&'a TruthTable> for PolarTruthTable {
    fn from(t: &TruthTable) -> Self {
        let newt = t.table.iter().map(|&i| if i { -1 } else { 1 }).collect();
        PolarTruthTable {
            table: newt,
            log2len: t.log2len,
        }
    }
}
impl PolarTruthTable {
    pub fn walsh_tform(&self) -> WalshTform {
        let mut truth = self.table.clone();
        walsh_tform_step(&mut truth);
        WalshTform {
            table: truth,
            log2len: self.log2len,
        }
    }
}

impl Display for TruthTable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (a, b) in BinaryStringIterator::new(self.log2len).zip(self.table.iter()) {
            let bin: Vec<u8> = a.iter().rev().map(|&i| if i { 1u8 } else { 0 }).collect();
            writeln!(f, "F{:?} = {}", bin, b)?;
        }
        Ok(())
    }
}
impl Display for PolarTruthTable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (a, b) in BinaryStringIterator::new(self.log2len).zip(self.table.iter()) {
            let bin: Vec<u8> = a.iter().rev().map(|&i| if i { 1u8 } else { 0 }).collect();
            writeln!(f, "F{:?} = {}", bin, b)?;
        }
        Ok(())
    }
}
impl Display for WalshTform {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (a, b) in BinaryStringIterator::new(self.log2len).zip(self.table.iter()) {
            let bin: Vec<u8> = a.iter().rev().map(|&i| if i { 1u8 } else { 0 }).collect();
            writeln!(f, "F{:?} = {}", bin, b)?;
        }
        Ok(())
    }
}

impl WalshTform {
    /* Deviation from correlation immunity
     */
    pub fn cidev(&self, k: usize) -> u32 {
        BinaryStringIterator::new(self.log2len)
            .filter(|i| {
                let w = hamming_weight(i);
                1 <= w && w <= k
            }).map(|omega| self.table[vec2usize(&omega)].abs())
            .max()
            .unwrap() as u32
    }
    pub fn radius(&self) -> i32 {
        self.table.iter().map(|i| i.abs()).max().unwrap()
    }
}
impl OArray {
    pub fn truth_table(&self) -> TruthTable {
        let l = 2usize.pow(self.k as u32);
        let mut out = vec![false; l];
        for r in self.iter_rows() {
            let val = r.iter().fold(0, |acc, &&val| acc * 2 + (val as usize));
            out[val] = true;
        }
        TruthTable::new(out)
    }
    pub fn from_truth_table(
        truth: &TruthTable,
        ngrande: usize,
        target_t: u32,
        fitness_f: FitnessFunction,
    ) -> Option<Self> {
        let k = truth.log2len;
        let rows: Vec<usize> = truth
            .table
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == true)
            .map(|(ind, _)| ind)
            .collect();
        if rows.len() != ngrande {
            return None;
        }
        let d: Vec<bool> = (1..=k)
            .flat_map(|col| rows.iter().map(move |row| row & (1 << (k - col)) != 0))
            .collect();
        Some(OArray::new(rows.len(), k, target_t, d, fitness_f))
    }
}

fn walsh_tform_step(v: &mut [i32]) {
    let half = v.len() / 2;
    for i in 0..half {
        let temp = v[i];
        v[i] += v[i + half];
        v[i + half] = temp - v[i + half];
    }
    if half > 1 {
        walsh_tform_step(&mut v[..half]);
        walsh_tform_step(&mut v[half..]);
    }
}

#[test]
fn test_truth() {
    use fitness::FitnessFunction;
    use rand::thread_rng;
    let mut rng = thread_rng();
    //let r = OArray::new_random_balanced(8, 4, 2, &mut rng, FitnessFunction::Delta);
    let mut test = [1, -1, -1, 1, -1, 1, 1, -1];
    println!("{:?}", test);
    println!("{:?}", walsh_tform_step(&mut test));
    println!("{:?}", test);
}

#[test]
fn test_from_truth() {
    use fitness::FitnessFunction;
    use rand::thread_rng;
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let r = OArray::new_random_balanced(8, 4, 2, &mut rng, FitnessFunction::Delta);
        let tt = r.truth_table();
        let r_new = OArray::from_truth_table(&tt, 8, 2, FitnessFunction::Delta);
        if let Some(r_new) = r_new {
            assert!(r_new.iter_rows().all(|r1| r.iter_rows().any(|r2| r1 == r2)));
        }
    }
}
