use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::iter::FromIterator;
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
       let mut ans = vec![];
       Solution::dfs(root, &mut ans);
       ans 
    } 
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none(){
            return;
        }
        let root = root.unwrap();
        Solution::dfs(root.borrow().left.clone(), ans);
        ans.push(root.borrow().val);
        Solution::dfs(root.borrow().right.clone(), ans);
    }
}
struct Solution;
fn main() {}
