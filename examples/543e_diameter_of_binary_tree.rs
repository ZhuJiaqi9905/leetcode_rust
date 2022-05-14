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

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root).1
    }
    /// Return max_depth, max_diameter
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(node) => {
                let left = Self::dfs(node.borrow().left.clone());
                let right = Self::dfs(node.borrow_mut().right.clone());
                // let max_depth = std::cmp::max(l.0, r.0) + 1;
                let max_depth = i32::max(left.0, right.0) + 1;
                let mut max_diameter = left.0 + right.0;
                max_diameter = max_diameter.max(left.1);
                max_diameter = max_diameter.max(right.1);
                (max_depth, max_diameter)
            }
            None => (0, 0),
        }
    }
}

struct Solution;
fn main() {
    let mut root = TreeNode::new(3);
    let node1 = TreeNode::new(9);
    let mut node2 = TreeNode::new(20);
    let node3 = TreeNode::new(15);
    let node4 = TreeNode::new(7);
    node2.left = Some(Rc::new(RefCell::new(node3)));
    node2.right = Some(Rc::new(RefCell::new(node4)));
    root.left = Some(Rc::new(RefCell::new(node1)));
    root.right = Some(Rc::new(RefCell::new(node2)));

    assert_eq!(
        3,
        Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))))
    );
}
