extern crate ctrlc;
extern crate evco;
extern crate oarray;
extern crate pbr;
extern crate rand;

use pbr::ProgressBar;

use ctrlc::set_handler;
use evco::gp::tree::*;
use evco::gp::*;
use rand::{OsRng, Rng};
use std::f64;
use std::sync::mpsc;

mod gpoarray;
mod treeformula;
use gpoarray::GPOArray;

fn main() {
    let n = 4;
    let k = 8;
    let t = 2;
    let max_depth = n;
    //let mut rng = OsRng::new().unwrap();
    //let tree_gen = TreeGen::full(&mut rng, 1, 4);

    let mut rng = OsRng::new().unwrap();
    let crossover = Crossover::one_point();

    let mut mutate_rng = OsRng::new().unwrap();
    let mut mut_tree_gen = TreeGen::full(&mut mutate_rng, 1, 2);
    let mutation = Mutation::uniform();

    let pop_size = 500;
    let mut population: Vec<GPOArray> = (0..pop_size)
        .map(|_| GPOArray::new_rand(n, k, t, max_depth, &mut rng))
        .collect();

    let epochs = 10000;
    let cmp_func =
        |a: &&GPOArray, b: &&GPOArray| a.fitness().partial_cmp(&b.fitness()).unwrap();

    let mut r = OsRng::new().unwrap();
    let mut pb = ProgressBar::new(epochs);

    let (tx, rx) = mpsc::channel();
    set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
    for _ in 0..epochs {
        let mut new_pop: Vec<GPOArray> = Vec::with_capacity(pop_size);
        {
            let old_best = population.iter().max_by(cmp_func).unwrap();
            let old_best_fitness = old_best.fitness();
            if -old_best_fitness < f64::EPSILON { break }
            pb.message(&format!(
                " Best:{:.4}, Mean: {:.4}, \n",
                old_best.fitness(),
                population.iter().map(|i| i.fitness()).sum::<f64>() / (pop_size as f64)
            ));
            new_pop.push(old_best.clone());
            let mut new_best_fitness = f64::NEG_INFINITY;
            for _ in 0..pop_size - 1 {
                let a = r.gen_range(0, pop_size);
                let mut b = a;
                while b == a {
                    b = r.gen_range(0, pop_size);
                }
                let mut c = b;
                while c == b || c == a {
                    c = r.gen_range(0, pop_size);
                }
                let mut tmp = [&population[a], &population[b], &population[c]];
                tmp.sort_by(cmp_func);
                let mut tmp2 = tmp[2].clone();
                tmp2.mate(&mut tmp[1].clone(), crossover, &mut r);
                if *r.choose(&[true,false]).unwrap() {
                    tmp2.mutate(&mut mut_tree_gen, mutation);
                }
                tmp2.update_fitness();
                new_best_fitness = new_best_fitness.max(tmp2.fitness());
                new_pop.push(tmp2);
            }
            if rx.try_recv().is_ok() {
                break;
            }
        }
        std::mem::swap(&mut population, &mut new_pop);
        pb.inc();
    }
    let best = population.iter().max_by(cmp_func).unwrap();
    if -best.fitness() < f64::EPSILON {
        println!("\n\n{}\n{}", best.to_oarray(), best);
        std::process::exit(2);
    } else {
        println!("Not found :(");
        std::process::exit(1);
    }
}
