use core::num;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::left_bound(&nums, target);
    }
    fn left_bound(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                right = mid - 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        // 出界的情况下，left值就是要插入的位置
        return left;
    }
}
struct Solution;
fn main() {}
