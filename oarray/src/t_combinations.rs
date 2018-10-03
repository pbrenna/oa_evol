//! Genera tutte le combinazioni di t elementi tra n,
//! utilizzando l'algoritmo L di Knuth

use streaming_iterator::StreamingIterator;

/// Struttura di appoggio per generare le combinazioni.
pub struct Combinations {
    /// numero elementi da cui generare le combinazioni
    n: usize,
    /// lunghezza delle combinazioni generate
    t: usize,
    /// vettore di appoggio
    c: Vec<usize>,
}

/// oggetto iteratore. Stato della generazione delle combinazioni
pub struct CombinationsIter<'a> {
    comb: &'a mut Combinations,
    initial: bool,
    end: bool,
}

impl Combinations where {
    pub fn new(n: usize, t: u32) -> Self {
        assert!(n >= t as usize);
        Combinations {
            n,
            t: t as usize,
            c: Vec::new(),
        }
    }
    /// Restituisce un iteratore sulle combinazioni
    pub fn stream_iter(&mut self) -> CombinationsIter {
        let mut v: Vec<usize> = (0..self.t).collect();
        v.push(self.n);
        v.push(0);
        self.c = v;
        CombinationsIter {
            comb: self,
            initial: true,
            end: false,
        }
    }
}



impl<'a> StreamingIterator for CombinationsIter<'a> {
    type Item = [usize];
    /// Algoritmo L di Knuth
    fn advance(&mut self) {
        if self.initial {
            if self.comb.t == 0 {
                self.end = true;
                return;
            }
            self.initial = false;
        } else {
            let mut j = 0usize;
            {
                let c = &mut self.comb.c;
                while c[j] + 1 == c[j + 1] {
                    let tmp = j;
                    c[j] = tmp;
                    j += 1;
                }
                c[j] += 1;
            }
            if j >= self.comb.t {
                self.end = true;
            }
        }
    }
    fn get(&self) -> Option<&Self::Item> {
        if self.end {
            None
        } else {
            Some(&self.comb.c[0..self.comb.t])
        }
    }
}

#[allow(dead_code)]
fn check_result(results: &[&[usize]], mut c: impl StreamingIterator<Item = [usize]>) {
    let mut i = 0;
    while let Some(item) = c.next() {
        assert!(item == results[i]);
        i += 1;
    }
    assert!(i == results.len());
}

#[test]
fn test0() {
    let mut c = Combinations::new(2, 0);
    let results: Vec<&[usize]> = vec![];
    check_result(&results, c.stream_iter());
}

#[test]
fn test1() {
    let mut c = Combinations::new(2, 1);
    let results: Vec<&[usize]> = vec![&[0], &[1]];
    check_result(&results, c.stream_iter());
}

#[test]
fn test2() {
    let mut c = Combinations::new(4, 3);
    let results: Vec<&[usize]> = vec![&[0usize, 1, 2], &[0, 1, 3], &[0, 2, 3], &[1, 2, 3]];
    check_result(&results, c.stream_iter());
}
