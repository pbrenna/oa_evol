/*
pub struct BinaryStringIterator {
    cur: usize,
    n: usize,
}
impl BinaryStringIterator {
    pub fn new(n: usize) -> Self {
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
}*/

pub struct BinaryStringIterator {
    cur: Vec<bool>,
    ended: bool,
}
impl BinaryStringIterator {
    pub fn new(n: usize) -> Self {
        BinaryStringIterator {
            cur: vec![false; n],
            ended: false,
        }
    }
}

impl Iterator for BinaryStringIterator {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        let ret = Some(self.cur.clone());
        for i in &mut self.cur {
            *i = !*i;
            if *i {
                return ret;
            }
        }
        self.ended = true;
        ret
    }
}
pub fn hamming_weight(v: &[bool]) -> usize {
    v.iter().filter(|&&v| v).count()
}
pub fn usize_hamming_weight(mut v: usize) -> usize {
    let mut out = 0;
    while v != 0 {
        if v & 1 != 0 {
            out += 1;
        }
        v >>= 1;
    }
    out
}
pub fn vec2usize(v: &[bool]) -> usize {
    v.iter().fold(0usize, |acc, &val| acc * 2 + val as usize)
}

#[test]
fn test123123() {
    let a = BinaryStringIterator::new(4);
    for x in a {
        println!("{:?}", x);
    }
}
#[test]
fn test_weight() {
    assert!(hamming_weight(&[false, true, false, false]) == 1);
    assert!(hamming_weight(&[true, true, true]) == 3);
    assert!(hamming_weight(&[false]) == 0);
    assert!(hamming_weight(&[false, false, true]) == 1);
}
#[test]
fn vec2u() {
    assert!(vec2usize(&[true, false, true]) == 5);
    assert!(vec2usize(&[true, true, false]) == 6);
    assert!(vec2usize(&[false, false, false]) == 0);
}

#[test]
fn usize_test(){
    assert!(usize_hamming_weight(10) == 2);
    assert!(usize_hamming_weight(7) == 3);
    assert!(usize_hamming_weight(128) == 1);
    assert!(usize_hamming_weight(0) == 0);
}