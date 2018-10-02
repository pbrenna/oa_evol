use evco::gp::tree::*;
use evco::gp::*;
use oarray::OArray;
use rand::Rng;
use treeformula::{TreeFormula, TreeFormulaConfig};
use std::fmt::{Display, Formatter,Error};

#[derive(Clone)]
pub struct GPOArray {
    pub trees: Vec<Individual<TreeFormula>>,
    ngrande: usize,
    n: usize,
    k: usize,
    target_t: u32,
    lazy_fitness: Option<f64>
}

struct BinaryStringIterator {
    cur: usize,
    n: usize,
}
impl BinaryStringIterator {
    fn new(n: usize) -> Self {
        BinaryStringIterator { cur: 0, n }
    }
}
impl Iterator for BinaryStringIterator {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        let max = 2usize.pow(self.n as u32);
        if self.cur < max {
            let mut out = Vec::with_capacity(self.n);
            let mut tmp = self.cur;
            for _ in 0..self.n {
                out.push((tmp & 1) == 1);
                tmp >>= 1;
            }
            self.cur += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl GPOArray {
    pub fn new_rand<R: Rng>(
        n: usize,
        k: usize,
        target_t: u32,
        max_depth: usize,
        rng: &mut R,
    ) -> Self {
        let mut trees = Vec::with_capacity(k);
        let mut tree_gen = TreeGen::perfect(rng, 1, max_depth);
        let ngrande = 2usize.pow(n as u32);
        let config = TreeFormulaConfig {
            n_variables: n,
        };
        for _ in 0..k {
            trees.push(Individual::new(&mut tree_gen, &config));
        }
        GPOArray {
            trees,
            ngrande,
            n,
            k,
            target_t,
            lazy_fitness: None
        }
    }
    pub fn to_oarray(&self) -> OArray {
        let mut oa_data = Vec::with_capacity(self.ngrande * self.k);
        for col in 0..self.k {
            for env in BinaryStringIterator::new(self.n) {
                oa_data.push(self.trees[col].tree.evaluate(&env));
            }
        }
        OArray::new(self.ngrande, self.k, self.target_t, oa_data)
    }
    pub fn fitness(&self) -> f64 {
        if let Some(fit) = self.lazy_fitness {
            fit
        } else {
            self.real_fitness()
        }
    }
    pub fn mate<R: Rng>(&mut self, other: &mut GPOArray, crossover: Crossover, rng: &mut R) {
        assert!(self.n == other.n);
        assert!(self.k == other.k);
        for (a, b) in self.trees.iter_mut().zip(other.trees.iter_mut()) {
            crossover.mate(a, b, rng);
        }
        self.lazy_fitness = None;
    }
    pub fn mutate<R: Rng>(&mut self, tg: &mut TreeGen<R>, mutation: Mutation) {
        let config = TreeFormulaConfig {
            n_variables: self.n,
        };
        for tree in self.trees.iter_mut() {
            mutation.mutate(tree, tg, &config);
        }
        self.lazy_fitness = None;
    }
    pub fn update_fitness(&mut self){
        self.lazy_fitness = Some(self.real_fitness());
    }
    pub fn real_fitness(&self) -> f64 {
        let oa = self.to_oarray();
        let delta_grande = oa.fitness();
        let mut tot = 0i64;
        for col in oa.iter_cols() {
            let mut acc = 0i64;
            for cell in col {
                if *cell {
                    acc += 1;
                } else {
                    acc -= 1;
                }
            }
            tot += acc.abs();
        }
        delta_grande - (tot as f64)
    }
}

impl Display for GPOArray {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (i,x) in self.trees.iter().enumerate() {
            writeln!(f, "Formula {}: {}", i, x.tree)?;
        }
        writeln!(f,"")
    }
}
