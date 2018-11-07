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
pub fn vec2usize(v: &[bool]) -> usize {
    v.iter().fold(0usize, |acc, &val| {acc*2 + val as usize})
}

#[test]
fn test123123() {
    let mut a = BinaryStringIterator::new(4);
    for x in a {
        println!("{:?}",x );
    }
}