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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        Solution::dfs(root, &mut v);
        v
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        ans.push(root.borrow().val);
        Solution::dfs(root.borrow().left.clone(), ans);
        Solution::dfs(root.borrow().right.clone(), ans);
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
        vec![3, 9, 20, 15, 7],
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(root))))
    );
}
