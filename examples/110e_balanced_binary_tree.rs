use std::cell::RefCell;
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_balance = true;
        Solution::depth(root, &mut is_balance);
        return is_balance;
    }
    fn depth(root: Option<Rc<RefCell<TreeNode>>>, is_balance: &mut bool) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let left_depth = Solution::depth(root.borrow().left.clone(), is_balance);
        let right_depth = Solution::depth(root.borrow().right.clone(), is_balance);
        if i32::abs(left_depth - right_depth) > 1 {
            *is_balance = false;
        }
        return 1 + i32::max(left_depth, right_depth);
    }
}
struct Solution;
fn main() {}
