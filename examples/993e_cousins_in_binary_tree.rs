use std::cell::RefCell;
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
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
   pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut x_parent = None;
        let mut y_parent = None;
        let mut x_depth = 0;
        let mut y_depth = 0;
        Solution::dfs(
            root,
            x,
            y,
            None,
            0,
            &mut x_parent,
            &mut y_parent,
            &mut x_depth,
            &mut y_depth,
        );
        if x_depth == y_depth && x_parent != y_parent {
            return true;
        }
        false
    }
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        x: i32,
        y: i32,
        parent: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        x_parent: &mut Option<Rc<RefCell<TreeNode>>>,
        y_parent: &mut Option<Rc<RefCell<TreeNode>>>,
        x_depth: &mut i32,
        y_depth: &mut i32,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        if root.borrow().val == x {
            *x_parent = parent;
            *x_depth = depth;
        } else if root.borrow().val == y {
            *y_parent = parent;
            *y_depth = depth;
        }

        Solution::dfs(
            root.borrow().left.clone(),
            x,
            y,
            Some(root.clone()),
            depth + 1,
            x_parent,
            y_parent,
            x_depth,
            y_depth,
        );
        Solution::dfs(
            root.borrow().right.clone(),
            x,
            y,
            Some(root.clone()),
            depth + 1,
            x_parent,
            y_parent,
            x_depth,
            y_depth,
        );
    }
}