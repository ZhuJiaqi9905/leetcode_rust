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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Solution::dfs(root, 0, &mut 0, &mut ans);
        ans
    }

    pub fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut depth: i32,
        max_depth: &mut i32,
        ans: &mut i32,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        depth += 1;
        if depth > *max_depth {
            *max_depth = depth;
            *ans = root.borrow().val;
        }
        Solution::dfs(root.borrow().left.clone(), depth, max_depth, ans);
        Solution::dfs(root.borrow().right.clone(), depth, max_depth, ans);
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

    assert_eq!(
        7,
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(node1))))
    );
}
