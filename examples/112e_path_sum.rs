use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut ans = false;
        Solution::dfs(root.unwrap(), target_sum, 0, &mut ans);
        ans
    }
    fn dfs(root: Rc<RefCell<TreeNode>>, target_sum: i32, mut sum: i32, ans: &mut bool) {
        if *ans {
            return;
        }
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        let val = root.borrow().val;
        if left.is_none() && right.is_none() {
            sum += val;
            if sum == target_sum {
                *ans = true;
            }
            return;
        }
        if left.is_some() {
            Solution::dfs(left.unwrap(), target_sum, sum + val, ans);
        }
        if right.is_some() {
            Solution::dfs(right.unwrap(), target_sum, sum + val, ans);
        }
    }
}
struct Solution;
fn main() {}
