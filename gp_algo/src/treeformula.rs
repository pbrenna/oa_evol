use evco::gp::tree::*;
use rand::Rng;
use std::char;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeFormula {
    Xor(BoxTree<TreeFormula>, BoxTree<TreeFormula>),
    If(
        BoxTree<TreeFormula>,
        BoxTree<TreeFormula>,
        BoxTree<TreeFormula>,
    ),
    And(BoxTree<TreeFormula>, BoxTree<TreeFormula>),
    Or(BoxTree<TreeFormula>, BoxTree<TreeFormula>),
    Not(BoxTree<TreeFormula>),
    Var(usize),
}

pub struct TreeFormulaConfig {
    pub n_variables: usize,
}

use self::TreeFormula::*;
impl Tree for TreeFormula {
    type Environment = Vec<bool>;
    type Action = bool;
    type Config = TreeFormulaConfig;

    fn branch<R: Rng>(
        tg: &mut TreeGen<R>,
        current_depth: usize,
        cfg: &Self::Config,
    ) -> BoxTree<Self> {
        let left = Self::child(tg, current_depth + 1, cfg);
        let right = Self::child(tg, current_depth + 1, cfg);
        match tg.gen_range(0, 5) {
            0 => Xor(left, right),
            1 => And(left, right),
            2 => Or(left, right),
            3 => Not(left),
            4 => If(Self::child(tg, current_depth + 1, cfg), left, right),
            _ => unreachable!(),
        }
        .into()
    }

    /// Generate tree leaves, only allowing the variables to take part.
    fn leaf<R: Rng>(tg: &mut TreeGen<R>, _: usize, cfg: &Self::Config) -> BoxTree<Self> {
        Var(tg.gen_range(0, cfg.n_variables)).into()
    }
    fn count_children(&mut self) -> usize {
        match self {
            Xor(_, _) | And(_, _) | Or(_, _) => 2,
            If(_, _, _) => 3,
            Not(_) => 1,
            Var(_) => 0,
        }
    }

    fn children(&self) -> Vec<&BoxTree<Self>> {
        match self {
            Xor(ref c1, ref c2) | And(ref c1, ref c2) | Or(ref c1, ref c2) => vec![c1, c2],
            Not(ref c1) => vec![c1],
            If(ref c1, ref c2, ref c3) => vec![c1, c2, c3],
            Var(_) => vec![],
        }
    }
    fn children_mut(&mut self) -> Vec<&mut BoxTree<Self>> {
        match self {
            Xor(ref mut c1, ref mut c2)
            | And(ref mut c1, ref mut c2)
            | Or(ref mut c1, ref mut c2) => vec![c1, c2],
            If(ref mut c1, ref mut c2, ref mut c3) => vec![c1, c2, c3],
            Not(ref mut c1) => vec![c1],
            Var(_) => vec![],
        }
    }

    fn evaluate(&self, env: &Self::Environment) -> bool {
        match self {
            Xor(ref a, ref b) => a.evaluate(env) ^ b.evaluate(env),
            And(ref a, ref b) => a.evaluate(env) && b.evaluate(env),
            Or(ref a, ref b) => a.evaluate(env) || b.evaluate(env),
            Not(ref a) => !a.evaluate(env),
            Var(i) => env[*i],
            If(ref cond, ref a, ref b) => {
                if cond.evaluate(env) {
                    a.evaluate(env)
                } else {
                    b.evaluate(env)
                }
            }
        }
    }
}

impl Display for TreeFormula {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Xor(ref a, ref b) => write!(f, "\\left({} \\oplus {}\\right)", a, b),
            And(ref a, ref b) => write!(f, "\\left({} \\land {}\\right)", a, b),
            Or(ref a, ref b) => write!(f, "\\left({} \\lor {}\\right)", a, b),
            Not(ref a) => write!(f, "\\neg {}", a),
            If(ref cond, ref a, ref b) => write!(
                f,
                "\\begin{{cases}}{} & \\text{{if }} {} \\\\ {} & \\text{{otherwise}}\\end{{cases}}",
                a, cond, b
            ),
            //Val(t) => write!(f, "{}", t.to_usize().unwrap()),
            Var(i) => write!(f, "x_{}", i),
        }
    }
}
