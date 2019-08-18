use super::OArray;
impl OArray {
    pub fn u_weight_d(&self, u: &[bool]) -> Vec<usize> {
        let mut distances = vec![0usize; self.k + 1];
        for i in self.iter_rows_val() {
            let d = hamming_dist(&i, u);
            debug_assert!(d <= self.k);
            distances[d] += 1;
        }
        distances
    }
    pub fn u_weight_d_col(&self, u: &[bool]) -> Vec<usize> {
        let mut distances = vec![0usize; self.ngrande + 1];
        for i in self.iter_cols() {
            let d = hamming_dist(i, u);
            debug_assert!(d <= self.ngrande);
            distances[d] += 1;
        }
        distances
    }
    pub fn zero_weight_d(&self) -> Vec<usize> {
        let zero = vec![false; self.k];
        self.u_weight_d(&zero)
    }
    pub fn proper_weight_d(&self) -> Vec<f64> {
        let mut tmp = vec![0usize; self.k + 1];
        for row in self.iter_rows_val() {
            let s = self.u_weight_d(&row);
            for(a,b) in tmp.iter_mut().zip(s.iter()) {
                *a += b;
            }
        }
        tmp.iter().map(|&i| i as f64 /self.ngrande as f64).collect()
    }
}

fn hamming_dist(a: &[bool], b: &[bool]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| if x == y { 0 } else { 1 })
        .sum()
}
#[test]
fn test_dist() {
    let a = [false, true, false];
    let b = [false, false, true];
    let c = [true, true, false];
    assert!(hamming_dist(&a, &a) == 0);
    assert!(hamming_dist(&a, &b) == 2);
    assert!(hamming_dist(&b, &a) == 2);
    assert!(hamming_dist(&a, &c) == 1);
    assert!(hamming_dist(&c, &a) == 1);
    assert!(hamming_dist(&b, &c) == 3);
    assert!(hamming_dist(&c, &b) == 3);
}

#[test]
fn test_zero_w() {
    use super::FitnessFunction;
    let d = vec![
        false, true, false, true, false, true, false, true, false, false, true, true, false, false,
        true, true, false, false, false, false, true, true, true, true,
    ];
    let result = vec![1, 3, 3, 1];
    let oa = OArray::new(8, 3, 2, d, FitnessFunction::Delta);
    println!("{}", oa);
    assert!(oa.zero_weight_d() == result);
}
