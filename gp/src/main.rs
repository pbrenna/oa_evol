extern crate evco;
extern crate num_iter;
extern crate oarray;
extern crate rand;

use evco::gp::tree::*;
use evco::gp::*;
use num_iter::range;
use rand::Rng;

use oarray::alphabet::Alphabet;
use oarray::OArray;

#[derive(Clone, Debug, PartialEq, Eq)]
enum TreeFormula<T: Alphabet> {
    AddMod(BoxTree<TreeFormula<T>>, BoxTree<TreeFormula<T>>),
    Var(usize),
    Val(T),
}

struct TreeFormulaConfig<T: Alphabet> {
    n_variables: usize,
    alphabet_max: T,
}

use self::TreeFormula::*;
impl<T: Alphabet> Tree for TreeFormula<T> {
    type Environment = (T, Vec<T>);
    type Action = T;
    type Config = TreeFormulaConfig<T>;

    fn branch<R: Rng>(
        tg: &mut TreeGen<R>,
        current_depth: usize,
        cfg: &Self::Config,
    ) -> BoxTree<Self> {
        let left = Self::child(tg, current_depth + 1, cfg);
        let right = Self::child(tg, current_depth + 1, cfg);
        match tg.gen_range(0, 1) {
            0 => AddMod(left, right),
            _ => unreachable!(),
        }
        .into()
    }

    fn leaf<R: Rng>(tg: &mut TreeGen<R>, _: usize, cfg: &Self::Config) -> BoxTree<Self> {
        let possible_vals = range(T::zero(), cfg.alphabet_max).collect::<Vec<T>>();
        match tg.gen_range(0, 2) {
            0 => Var(tg.gen_range(0, cfg.n_variables)),
            1 => Val(*tg.choose(&possible_vals).unwrap()),
            _ => unreachable!(),
        }
        .into()
    }
    fn count_children(&mut self) -> usize {
        match self {
            AddMod(_, _) => 2,
            _ => 0,
        }
    }

    fn children(&self) -> Vec<&BoxTree<Self>> {
        match self {
            AddMod(ref c1, ref c2) => vec![c1, c2],
            _ => vec![],
        }
    }
    fn children_mut(&mut self) -> Vec<&mut BoxTree<Self>> {
        match self {
            AddMod(ref mut c1, ref mut c2) => vec![c1, c2],
            _ => vec![],
        }
    }

    fn evaluate(&self, env: &Self::Environment) -> T {
        let vars = &env.1;
        let out = match self {
            AddMod(ref a, ref b) => a.evaluate(env) + b.evaluate(env),
            Val(t) => *t,
            Var(i) => vars[*i]
        };
        out % env.0
    }
}

fn main() {}
