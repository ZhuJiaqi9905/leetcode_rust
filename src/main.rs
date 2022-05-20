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
    pub fn stone_game(piles: Vec<i32>) -> bool {
        return true;
    }
}
struct Solution;
fn main() {}
