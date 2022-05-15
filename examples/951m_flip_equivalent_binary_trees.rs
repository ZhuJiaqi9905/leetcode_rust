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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::dfs(root1, root2)
    }
    fn dfs(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if (root1.is_none() && root2.is_some()) || (root1.is_some() && root2.is_none()) {
            return false;
        }
        if root1.is_none() && root2.is_none() {
            return true;
        }
        let root1 = root1.unwrap();
        let root2 = root2.unwrap();
        if root1.borrow().val != root2.borrow().val {
            return false;
        }
        return (Solution::dfs(root1.borrow().left.clone(), root2.borrow().left.clone())
            && Solution::dfs(root1.borrow().right.clone(), root2.borrow().right.clone()))
            || (Solution::dfs(root1.borrow().left.clone(), root2.borrow().right.clone())
                && Solution::dfs(root1.borrow().right.clone(), root2.borrow().left.clone()));
    }
}
