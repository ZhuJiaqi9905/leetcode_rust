struct Solution;

fn main() {
    let digits: Vec<i32> = vec![9, 9, 9];
    println!("{:?}", Solution::plus_one(digits));
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.clone();
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
            if i > 0 {
                i -= 1;
            } else if i == 0 {
                break;
            }
        }
        digits = vec![0; digits.len() + 1];
        digits[0] = 1;
        digits
    }
}
