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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if !(p.is_some() && q.is_some()) {
            return false;
        }
        let p = p.unwrap();
        let q = q.unwrap();
        if p.borrow().val != q.borrow().val {
            return false;
        }
        return Solution::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
            && Solution::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
    }
}
struct Solution;
fn main() {}
