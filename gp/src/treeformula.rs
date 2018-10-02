use std::cmp::max;
use std::cmp::min;
use evco::gp::tree::*;
use num_iter::range;
use oarray::alphabet::Alphabet;
use rand::Rng;
use std::char;
use std::fmt::{Display, Error, Formatter};
use std::marker;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeFormula<T: Alphabet> {
    AddMod(BoxTree<TreeFormula<T>>, BoxTree<TreeFormula<T>>),
    If(
        BoxTree<TreeFormula<T>>,
        BoxTree<TreeFormula<T>>,
        BoxTree<TreeFormula<T>>,
    ),
    Min(BoxTree<TreeFormula<T>>, BoxTree<TreeFormula<T>>),
    Max(BoxTree<TreeFormula<T>>, BoxTree<TreeFormula<T>>),
    Neg(BoxTree<TreeFormula<T>>),
    Var(usize),
    Val(T),
}

pub struct TreeFormulaConfig<T: Alphabet> {
    pub n_variables: usize,
    pub alphabet_max: T,
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
        let cond = Self::child(tg, current_depth + 1, cfg);
        match tg.gen_range(0, 5) {
            0 => AddMod(left, right),
            1 => Min(left, right),
            2 => Max(left, right),
            3 => Neg(left),
            4 => If(cond, left, right),
            _ => unreachable!(),
        }
        .into()
    }

    /// Generate tree leaves, only allowing the variables to take part.
    fn leaf<R: Rng>(tg: &mut TreeGen<R>, _: usize, cfg: &Self::Config) -> BoxTree<Self> {
        let possible_vals = range(T::zero(), cfg.alphabet_max).collect::<Vec<T>>();
        /*match tg.gen_range(0, 2) {
            0 => Var(tg.gen_range(0, cfg.n_variables)),
            1 => Val(*tg.choose(&possible_vals).unwrap()),
            _ => unreachable!(),
        }
        .into()*/
        Var(tg.gen_range(0, cfg.n_variables)).into()
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
            Min(ref a, ref b) => min(a.evaluate(env), b.evaluate(env)),
            Max(ref a, ref b) => max(a.evaluate(env), b.evaluate(env)),
            Neg(ref a) => env.0 - a.evaluate(env),
            Val(t) => *t,
            Var(i) => vars[*i],
            If(ref cond, ref a, ref b) => {
                if cond.evaluate(env)!= T::zero() {
                    a.evaluate(env)
                } else {
                    b.evaluate(env)
                }
            }
        };
        out % env.0
    }
}

impl<T: Alphabet> Display for TreeFormula<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            AddMod(ref a, ref b) => write!(f, "{} ⨁  {}", a, b),
            Min(ref a, ref b) => write!(f, "({} ∧ {})", a, b),
            Max(ref a, ref b) => write!(f, "({} ∨ {})", a, b),
            Neg(ref a) => write!(f, "¬ ({})", a),
            If(ref cond, ref a, ref b) => write!(f, "if ({}, then {}, else {})", cond, a, b),
            //Val(t) => write!(f, "{}", t.to_usize().unwrap()),
            Var(i) => write!(f, "x{}", subscript(*i)),
            _ => unimplemented!(),
        }
    }
}
fn subscript(i: usize) -> String {
    if i >= 10 {
        subscript(i / 10) + &subscript(i % 10)
    } else {
        char::from_u32(i as u32 + 0x2080).unwrap().to_string()
    }
}
