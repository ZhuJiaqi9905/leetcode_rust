use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();
        for i in 0..nums.len() {
            push(&mut deque, nums[i]);
            if (i as i32) == k - 1 {
                ans.push(*deque.front().unwrap());
            } else if (i as i32) > k - 1 {
                if !deque.is_empty() && *deque.front().unwrap() == nums[i - k as usize] {
                    deque.pop_front();
                }
                ans.push(*deque.front().unwrap());
            } 
        }
        ans
    }
}

fn push(deque: &mut VecDeque<i32>, n: i32) {
    while !deque.is_empty() && *deque.back().unwrap() < n {
        deque.pop_back();
    }
    deque.push_back(n);
}

fn main() {
    let nums: Vec<i32> = vec!(1,3,-1,-3,5,3,6,7);
    let ans  = Solution::max_sliding_window(nums, 3);
    println!("{:?}", ans);
}
