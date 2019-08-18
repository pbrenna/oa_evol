use fitness::FitnessFunction;
use oarray::OArray;

impl<'a> From<&'a str> for OArray {
    fn from(input: &'a str) -> Self {
        let rows = input.trim().lines();
        let mut d = Vec::new();
        let mut k = None;
        let mut row_cnt = 0;
        for row in rows {
            let mut nums: Vec<bool> = row
                .split_whitespace()
                .map(|i| match i {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Unexpected char"),
                })
                .collect();
            if k.is_some() && nums.len() != k.unwrap() {
                panic!("Uneven column number across lines.");
            }
            k = Some(nums.len());
            d.append(&mut nums);
            row_cnt += 1;
        }
        let k = k.unwrap();

        //Transform to col-major mode
        let mut d1 = Vec::with_capacity(row_cnt * k);
        for j in 0..k {
            for i in 0..row_cnt {
                d1.push(d[k * i + j]);
            }
        }
        OArray::new(row_cnt, k, 1, d1, FitnessFunction::DeltaFast)
    }
}
#[test]
fn test_parse() {
    let string = "0 0 0
                  1 0 1
                  0 1 0
                  1 0 0";
    let oa = OArray::from(string);
    println!("{}", oa);
}

#[should_panic]
#[test]
fn test_parse2() {
    let string = "0 0 0
                  1 0 1
                  0 1 
                  1 0 0";
    let _ = OArray::from(string);
}
#[should_panic]
#[test]
fn test_parse3() {
    let string = "0 0 0
                  1 0 1
                  0 1 2
                  1 0 0";
    let _ = OArray::from(string);
}
