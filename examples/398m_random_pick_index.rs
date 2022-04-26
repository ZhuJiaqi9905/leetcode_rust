use rand::{thread_rng, Rng};
use std::collections::HashMap;
struct Solution {
    idx: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut idx = HashMap::<i32, Vec<i32>>::with_capacity(nums.len());
        for i in 0..nums.len() {
            let x = nums[i];
            match idx.get_mut(&x) {
                Some(v) => v.push(i as i32),
                None => {
                    idx.insert(x, vec![i as i32]);
                }
            }
        }
        Self { idx }
    }

    fn pick(&self, target: i32) -> i32 {
        let v = self.idx.get(&target).unwrap();
        let mut rng = thread_rng();
        if v.len() == 1 {
            v[0]
        } else {
            v[rng.gen_range(0..v.len())]
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
fn main() {
    let obj = Solution::new(vec![1, 2, 3, 3, 3]);
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(1));
    println!("{}", obj.pick(3));
}
