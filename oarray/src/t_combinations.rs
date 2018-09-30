//! Genera tutte le combinazioni di t elementi tra n,
//! utilizzando l'algoritmo L di Knuth

/// Struttura di appoggio per generare le combinazioni.
pub struct Combinations {
    /// numero elementi da cui generare le combinazioni
    n : usize,
    /// lunghezza delle combinazioni generate
    t: usize,
    /// vettore di appoggio
    c: Vec<usize>,
}

/// oggetto iteratore. Stato della generazione delle combinazioni
pub struct CombinationsIter<'a> {
    comb: &'a mut Combinations,
    initial: bool,
}

impl Combinations
where
{
    pub fn new(n: usize, t: usize) -> Self {
        Combinations { n, t, c: Vec::new() }
    }
    /// Restituisce un iteratore sulle combinazioni
    pub fn iter(&mut self) -> CombinationsIter {
        let mut v: Vec<usize> = (0 .. self.t).collect();
        v.push(self.n);
        v.push(0);
        self.c = v;
        CombinationsIter {
            comb: self,
            initial: true,
        }
    }
}

impl<'a> Iterator for CombinationsIter<'a>
{
    type Item = Vec<usize>;
    /// Algoritmo L di Knuth
    fn next(&mut self) -> Option<Self::Item> {
        if self.initial {
            if self.comb.t == 0
                { return None; }
            self.initial = false;
        } else {

            let mut j = 0usize;
            {
                let c = &mut self.comb.c;
                while c[j] + 1 == c[j+1] {
                    let tmp = j;
                    c[j] = tmp;
                    j += 1;
                }
                c[j] += 1;
            }
            if j >= self.comb.t {
                return None;
            } 
        }
        Some(Vec::from(&self.comb.c[0..self.comb.t]))
    }
}

#[allow(dead_code)]
fn check_result(results: &[&[usize]], c: impl Iterator<Item = Vec<usize>>){
    let mut tot = 0;
    for (i, result) in c.enumerate() {
        assert!(result == results[i]);
        tot += 1;
    }
    assert!(tot == results.len());
}

#[test]
fn test0(){
    let mut c = Combinations::new(2, 0);
    let results: Vec<&[usize]> = vec![];
    check_result(&results, c.iter());
}


#[test]
fn test1(){
    let mut c = Combinations::new(2, 1);
    let results: Vec<&[usize]> = vec![
        &[0],
        &[1]
    ];
    check_result(&results, c.iter());
}


#[test]
fn test2() {
    let mut c = Combinations::new(4, 3);
    let results : Vec<&[usize]> = vec![
        &[0usize, 1, 2],
        &[0, 1, 3],
        &[0, 2, 3],
        &[1, 2, 3]
    ];
    check_result(&results, c.iter());
}