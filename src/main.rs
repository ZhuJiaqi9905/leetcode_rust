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

struct Solution;
fn main() {
    let mut node1 = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let mut node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let mut node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);
    let node7 = TreeNode::new(7);
    node5.left = Some(Rc::new(RefCell::new(node7)));
    node3.left = Some(Rc::new(RefCell::new(node5)));
    node3.right = Some(Rc::new(RefCell::new(node6)));
    node2.left = Some(Rc::new(RefCell::new(node4)));
    node1.left = Some(Rc::new(RefCell::new(node2)));
    node1.right = Some(Rc::new(RefCell::new(node3)));
}
