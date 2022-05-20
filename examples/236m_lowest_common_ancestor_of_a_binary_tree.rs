use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;
use std::time::Instant;
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        if root == p || root == q {
            return root;
        }
        let root = root.unwrap();
        let left =
            Solution::lowest_common_ancestor(root.borrow().left.clone(), p.clone(), q.clone());
        let right = Solution::lowest_common_ancestor(root.borrow().right.clone(), p, q);
        if left.is_some() && right.is_some() {
            return Some(root);
        }
        if left.is_none() {
            return right;
        } else {
            return left;
        }
    }
}
struct Solution;
fn main() {}
