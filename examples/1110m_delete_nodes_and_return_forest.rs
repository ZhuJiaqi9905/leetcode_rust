use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::iter::FromIterator;
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
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if root.is_none() {
            return vec![];
        }
        let del_set = HashSet::from_iter(to_delete.into_iter());
        let mut ans = vec![];
        Solution::dfs(root, &del_set, &mut ans, false);
        ans
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        del_set: &HashSet<i32>,
        ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        has_parent: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        let deleted = del_set.contains(&root.borrow().val);
        if !deleted && !has_parent {
            //  没有父节点且不需要删除，说明是一个新的root
            ans.push(Some(root.clone()));
        }
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        root.borrow_mut().left = Solution::dfs(left, del_set, ans, !deleted);

        root.borrow_mut().right = Solution::dfs(right, del_set, ans, !deleted);
        // 如果需要被删除，返回 null 给父节点
        if deleted {
            return None;
        } else {
            return Some(root);
        }
    }
}

struct Solution;
fn main(){
    
}