use evco::gp::tree::*;
use evco::gp::*;
use oarray::{alphabet::Alphabet, OArray};
use rand::Rng;
use treeformula::{TreeFormula, TreeFormulaConfig};
use std::fmt::{Display, Formatter,Error};

#[derive(Clone)]
pub struct GPOArray<T: Alphabet> {
    pub trees: Vec<Individual<TreeFormula<T>>>,
    ngrande: usize,
    n: usize,
    k: usize,
    s: T,
    target_t: usize,
    lazy_fitness: Option<f64>
}

struct StringIterator<T> {
    s: T,
    cur: usize,
    n: usize,
}
impl<T: Alphabet> StringIterator<T> {
    fn new(s: T, n: usize) -> Self {
        StringIterator { cur: 0, s, n }
    }
}
impl<T: Alphabet> Iterator for StringIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let s_usize = self.s.to_usize().unwrap();
        let max = s_usize.pow(self.n as u32);
        if self.cur < max {
            let mut out = Vec::with_capacity(self.n);
            let mut tmp = self.cur;
            for _ in 0..self.n {
                out.push(T::from_usize(tmp % s_usize).unwrap());
                tmp /= s_usize;
            }
            self.cur += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl<T: Alphabet> GPOArray<T> {
    pub fn new_rand<R: Rng>(
        n: usize,
        k: usize,
        s: T,
        target_t: usize,
        max_depth: usize,
        rng: &mut R,
    ) -> Self {
        let mut trees = Vec::with_capacity(k);
        let mut tree_gen = TreeGen::full(rng, 1, max_depth);
        let ngrande = s.to_usize().unwrap().pow(n as u32);
        let config = TreeFormulaConfig {
            n_variables: n,
            alphabet_max: s,
        };
        for _ in 0..k {
            trees.push(Individual::new(&mut tree_gen, &config));
        }
        GPOArray {
            trees,
            ngrande,
            n,
            k,
            s,
            target_t,
            lazy_fitness: None
        }
    }
    pub fn to_oarray(&self) -> OArray<T> {
        let mut oa_data = Vec::with_capacity(self.ngrande * self.k);
        for col in 0..self.k {
            for env in StringIterator::new(self.s, self.n) {
                oa_data.push(self.trees[col].tree.evaluate(&(self.s, env)));
            }
        }
        OArray::new(self.ngrande, self.k, self.s, self.target_t, oa_data)
    }
    pub fn fitness(&self) -> f64 {
        if let Some(fit) = self.lazy_fitness {
            fit
        } else {
            let f = self.to_oarray().fitness();
            f
        }
    }
    pub fn mate<R: Rng>(&mut self, other: &mut GPOArray<T>, crossover: Crossover, rng: &mut R) {
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
            alphabet_max: self.s,
        };
        for tree in self.trees.iter_mut() {
            mutation.mutate(tree, tg, &config);
        }
        self.lazy_fitness = None;
    }
    pub fn update_fitness(&mut self){
        self.lazy_fitness = Some(self.to_oarray().fitness());
    }
}

impl<T: Alphabet> Display for GPOArray<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (i,x) in self.trees.iter().enumerate() {
            writeln!(f, "Formula {}: {}", i, x.tree)?;
        }
        writeln!(f,"")
    }
}

#[test]
fn strings() {
    let it = StringIterator::new(3u8, 4);
    for x in it {
        println!("{:?}", x);
    }
}
