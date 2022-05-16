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
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = 0;
        Solution::dfs(root, &mut max_sum);
        max_sum
    }
    // 要注意BST的成立条件
    // 注意i32的最小值是负数，不是0
    /// Returns (is_bst, sum_val_of_the_subtree, max_val_of_the_subtree, min_val_of_the_subtree)
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (bool, i32, i32, i32) {
        if root.is_none() {
            return (true, 0, i32::MIN, i32::MAX);
        }
        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        let left_ret = Solution::dfs(left.clone(), max_sum);
        let right_ret = Solution::dfs(right.clone(), max_sum);

        let is_bst_order = left_ret.2 < root.borrow().val && root.borrow().val < right_ret.3;
        let is_bst = left_ret.0 && right_ret.0 && is_bst_order;
        let sum = left_ret.1 + right_ret.1 + root.borrow().val;
        if is_bst && *max_sum < sum {
            *max_sum = sum;
        }
        let mut max_val = i32::max(root.borrow().val, left_ret.2);
        max_val = max_val.max(right_ret.2);
        let mut min_val = i32::min(root.borrow().val, left_ret.3);
        min_val = min_val.min(right_ret.3);
        return (is_bst, sum, max_val, min_val);
    }
}

struct Solution;
fn main() {
    let mut node1 = TreeNode::new(1);
    let mut node2 = TreeNode::new(10);
    let mut node3 = TreeNode::new(-5);
    let node4 = TreeNode::new(20);

    node2.left = Some(Rc::new(RefCell::new(node3)));
    node2.right = Some(Rc::new(RefCell::new(node4)));
    node1.right = Some(Rc::new(RefCell::new(node2)));
    println!(
        "{}",
        Solution::max_sum_bst(Some(Rc::new(RefCell::new(node1))))
    );
}
