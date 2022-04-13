use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            let x = nums[i];

            match m.get(&(target - x)) {
                Some(j) => {
                    ans.push(i as i32);
                    ans.push(*j);
                    break;
                }
                None => {
                    m.insert(x, i as i32);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}
