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
struct FindElements {
    values: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            return Self {
                values: HashSet::new(),
            };
        }
        let root = root.unwrap();
        let mut values = HashSet::new();
        FindElements::dfs(root, 0, &mut values);
        Self { values }
    }
    fn dfs(root: Rc<RefCell<TreeNode>>, value: i32, values: &mut HashSet<i32>) {
        root.borrow_mut().val = value;
        values.insert(value);
        if root.borrow().left.is_some() {
            FindElements::dfs(root.borrow().left.clone().unwrap(), 2 * value + 1, values);
        }
        if root.borrow().right.is_some() {
            FindElements::dfs(root.borrow().right.clone().unwrap(), 2 * value + 2, values);
        }
    }
    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
fn main() {}
