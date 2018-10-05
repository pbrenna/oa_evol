use evco::gp::tree::*;
use evco::gp::*;
use oarray::OArray;
use rand::Rng;
use spiril::unit::Unit;
use std::fmt::{Display, Error, Formatter};
use treeformula::{TreeFormula, TreeFormulaConfig};

#[derive(Clone)]
pub struct GPOArray<R: Rng> {
    pub trees: Vec<Individual<TreeFormula>>,
    ngrande: usize,
    n: usize,
    k: usize,
    target_t: u32,
    lazy_fitness: Option<f64>,
    crossover: Crossover,
    mutation: Mutation,
    mutation_prob: f64,
    tree_gen: TreeGen<R>,
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

impl<R: Rng + Send> GPOArray<R> {
    pub fn new_rand(
        n: usize,
        k: usize,
        target_t: u32,
        max_depth: usize,
        rng: R,
        crossover: Crossover,
        mutation: Mutation,
        mutation_prob: f64
    ) -> Self {
        let mut trees = Vec::with_capacity(k);
        let mut tree_gen = TreeGen::perfect(rng, 1, max_depth);
        let ngrande = 2usize.pow(n as u32);
        let config = TreeFormulaConfig { n_variables: n };
        for _ in 0..k {
            trees.push(Individual::new(&mut tree_gen, &config));
        }
        GPOArray {
            trees,
            ngrande,
            n,
            k,
            target_t,
            lazy_fitness: None,
            crossover,
            mutation,
            tree_gen,
            mutation_prob
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
    pub fn mutate(&mut self) {
        let config = TreeFormulaConfig {
            n_variables: self.n,
        };
        for tree in self.trees.iter_mut() {
            self.mutation.mutate(tree, &mut self.tree_gen, &config);
        }
        self.lazy_fitness = None;
    }
}

impl<R: Rng + Send + Clone> Unit for GPOArray<R> {
    fn fitness(&self) -> f64 {
        let oa = self.to_oarray();
        let delta_grande = oa.delta_fitness();
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
    fn breed_with(&self, other: &Self) -> Self {
        assert!(self.n == other.n);
        assert!(self.k == other.k);
        let mut a = self.clone();
        let mut b = other.clone();
        let mut other_tree_gen = self.tree_gen.clone();
        for (tree_a, tree_b) in a.trees.iter_mut().zip(b.trees.iter_mut()) {
            self.crossover.mate(tree_a, tree_b, &mut other_tree_gen);
        }
        if other_tree_gen.gen_bool(self.mutation_prob) {
            a.mutate();
        }
        a
    }
}

impl<R: Rng + Send> Display for GPOArray<R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (i, x) in self.trees.iter().enumerate() {
            writeln!(f, "Formula {}: {}", i, x.tree)?;
        }
        writeln!(f, "")
    }
}
