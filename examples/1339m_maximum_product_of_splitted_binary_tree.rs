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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_product = 0;
        let tree_sum = Solution::get_sum(root.clone(), 0, &mut max_product);
        max_product = 0;
        Solution::get_sum(root, tree_sum, &mut max_product);
        return (max_product % (1e9 as i64 + 7)) as i32;
    }
    fn get_sum(root: Option<Rc<RefCell<TreeNode>>>, tree_sum: i32, max_product: &mut i64) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let left_sum = Solution::get_sum(root.borrow().left.clone(), tree_sum, max_product);
        let right_sum = Solution::get_sum(root.borrow().right.clone(), tree_sum, max_product);
        let sum = left_sum + right_sum + root.borrow().val;
        *max_product = i64::max(*max_product, sum as i64 * (tree_sum - sum) as i64);
        return sum;
    }
}
struct Solution;
fn main() {}
