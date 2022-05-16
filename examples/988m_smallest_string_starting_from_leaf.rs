use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
// Definition for a binary tree node.
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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut tmp_str = vec![];
        let mut ans = String::new();
        Solution::dfs(root, &mut tmp_str, &mut ans);
        ans
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, tmp_str: &mut Vec<u8>, ans: &mut String) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        if left.is_none() && right.is_none() {
            tmp_str.push('a' as u8 + root.borrow().val as u8);
            tmp_str.reverse();

            let tmp_ans = std::str::from_utf8(tmp_str).unwrap();

            if tmp_ans < ans || ans == "" {
                *ans = tmp_ans.to_string();
            }
            tmp_str.reverse();
            tmp_str.pop();
            return;
        }
        tmp_str.push('a' as u8 + root.borrow().val as u8);
        Solution::dfs(left, tmp_str, ans);
        Solution::dfs(right, tmp_str, ans);
        tmp_str.pop();
    }
}

struct Solution;
fn main() {}
