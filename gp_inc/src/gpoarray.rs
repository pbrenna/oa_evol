use evco::gp::tree::*;
use evco::gp::*;
use oarray::binary_strings::BinaryStringIterator;
use oarray::wtform::PolarTruthTable;
use oarray::{
    FitnessFunction::{self, *},
    OArray,
};
use rand::Rng;
use spiril::unit::Unit;
use std::fmt::{Display, Error, Formatter};
use treeformula::{TreeFormula, TreeFormulaConfig};

#[derive(Clone)]
pub struct IncGPOArray<'a, R: Rng> {
    partial: &'a OArray,
    pub tree: Individual<TreeFormula>,
    ngrande: usize,
    n: usize,
    k: usize,
    target_t: u32,
    lazy_fitness: Option<f64>,
    crossover: Crossover,
    mutation: Mutation,
    mutation_prob: f64,
    tree_gen: TreeGen<R>,
    fitness_f: FitnessFunction,
}

impl<'a, R: Rng + Send> IncGPOArray<'a, R> {
    pub fn new_rand(
        partial: &'a OArray,
        ngrande: usize,
        k: usize,
        target_t: u32,
        max_depth: usize,
        rng: R,
        crossover: Crossover,
        mutation: Mutation,
        mutation_prob: f64,
        fitness_f: FitnessFunction,
    ) -> Self {
        let mut tree_gen = TreeGen::perfect(rng, 1, max_depth);
        let n = (ngrande as f64).log2().ceil() as usize;
        let config = TreeFormulaConfig { n_variables: n };
        let tree = Individual::new(&mut tree_gen, &config);
        IncGPOArray {
            partial,
            tree,
            ngrande,
            n,
            k,
            target_t,
            lazy_fitness: None,
            crossover,
            mutation,
            tree_gen,
            mutation_prob,
            fitness_f,
        }
    }
    pub fn to_oarray(&self) -> OArray {
        let mut oa = self.partial.clone();
        oa.k += 1;
        oa.d.extend(
            BinaryStringIterator::new(self.n)
                .take(self.ngrande)
                .map(|env| self.tree.tree.evaluate(&env)),
        );
        oa
    }
    pub fn mutate(&mut self) {
        let config = TreeFormulaConfig {
            n_variables: self.n,
        };
        self.mutation
            .mutate(&mut self.tree, &mut self.tree_gen, &config);
        self.lazy_fitness = None;
    }
    fn oa_fitness(&self, last_col: &[bool]) -> f64 {
        match self.partial.fitness_f {
            Walsh(x) => self.partial.walsh_incremental(x, last_col),
            WalshFaster(x) => self.partial.walsh_incremental_faster(x, last_col),
            Comb(x) => {
                let a = -self.partial.walsh_incremental_faster(2, last_col);
                let oa = self.to_oarray();
                let b = PolarTruthTable::from(&oa.truth_table()).walsh_tform().nonlinearity();
                //dbg!(b);
                -(a + f64::max(0.0, f64::from(x) * (self.k as f64 -1.0)-1.0-b))
            }
            DeltaFast => self.to_oarray().delta_incremental_faster(),
            _ => self.to_oarray().fitness(),
        }
    }
}

impl<'a, R: Rng + Send + Clone> Unit for IncGPOArray<'a, R> {
    fn fitness(&self) -> f64 {
        let last_col: Vec<bool> = BinaryStringIterator::new(self.n)
            .take(self.ngrande)
            .map(|env| self.tree.tree.evaluate(&env))
            .collect();
        let oa_fit = self.oa_fitness(&last_col);
        match self.partial.fitness_f {
            Walsh(_) | WalshFaster(_) => return oa_fit,
            _ => {}
        }
        let mut acc = 0i64;
        for cell in last_col {
            if cell {
                acc += 1;
            } else {
                acc -= 1;
            }
        }
        oa_fit - (acc.abs() as f64)
    }
    fn breed_with(&self, other: &Self) -> Self {
        assert!(self.n == other.n);
        assert!(self.k == other.k);
        let mut a = self.clone();
        let mut b = other.clone();
        let mut other_tree_gen = self.tree_gen.clone();
        self.crossover
            .mate(&mut a.tree, &mut b.tree, &mut other_tree_gen);
        if other_tree_gen.gen_bool(self.mutation_prob) {
            a.mutate();
        }
        a
    }
}

impl<'a, R: Rng + Send> Display for IncGPOArray<'a, R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Formula: {}", self.tree)?;
        writeln!(f, "")
    }
}
