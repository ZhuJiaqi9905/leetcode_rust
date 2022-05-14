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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = Self::max_depth(node.borrow().left.clone());
                let right = Self::max_depth(node.borrow_mut().right.clone());
                i32::max(left, right) + 1
            }
            None => 0,
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

    assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(root)))));
}
