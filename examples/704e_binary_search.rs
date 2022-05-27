use core::num;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use std::rc::Rc;
/// 搜素一个元素时，搜索区间两端闭。
/// while循环带等号，否则需要打补丁。
/// mid必须加减1，因为区间两端闭。
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = (nums.len() - 1) as i32;
        // 因为right可能为-1，所以要用有符号的整数。
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return -1;
    }
}
struct Solution;
fn main() {}
