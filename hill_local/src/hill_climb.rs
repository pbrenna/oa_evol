use oarray::binary_strings;
use oarray::wtform::*;
use oarray::OArray;

pub fn hill_climb(input: OArray) -> (OArray, bool) {
    assert!(input.k < 64);
    let mut truth = input.truth_table();
    if let Some(_idem) =
        OArray::from_truth_table(&truth, input.ngrande, input.target_t, input.fitness_f)
    {
    } else {
        return (input, false);
    }
    let wtform = PolarTruthTable::from(&truth).walsh_tform();
    let cidev_t = wtform.cidev(input.target_t as usize) as i32;

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
            if binary_strings::usize_hamming_weight(index) <= input.target_t as usize {
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
        //let old_f = input.fitness();
        truth.table.swap(i, j);
        //let old_oa = input.clone();
        if let Some(new) =
            OArray::from_truth_table(&truth, input.ngrande, input.target_t, input.fitness_f)
        {
            (new, true)
        } else {
            (input, false)
        }
    } else {
        (input, false)
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
