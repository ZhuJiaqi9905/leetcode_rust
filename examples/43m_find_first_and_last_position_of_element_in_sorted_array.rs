use core::num;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Solution::left_bound(&nums, target);
        let right = Solution::right_bound(&nums, target);
        return vec![left, right];
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
        // 检查出界情况
        if left >= nums.len() as i32 || nums[left as usize] != target {
            return -1;
        }
        return left;
    }
    fn right_bound(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                left = mid + 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        // 检查出界情况
        if right < 0 || nums[right as usize] != target {
            return -1;
        }
        return right;
    }
}
struct Solution;
fn main() {}
