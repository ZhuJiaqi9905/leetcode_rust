use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;
use std::time::Instant;
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
struct Ans {
    node: Option<Rc<RefCell<TreeNode>>>,// 最近公共祖先节点
    depth: i32,
}
impl Ans {
    pub fn new(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> Self {
        Self { node, depth }
    }
}
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::max_depth(root).node;
    }

    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> Ans {
        if root.is_none() {
            return Ans::new(None, 0);
        }
        let root = root.unwrap();
        let left = Solution::max_depth(root.borrow().left.clone());
        let right = Solution::max_depth(root.borrow().right.clone());
        if left.depth == right.depth {
            return Ans::new(Some(root), left.depth + 1);
        }
        // 左右子树的深度不同，则最近公共祖先在 depth 较大的一边
        let mut res = if left.depth > right.depth {
            left
        } else {
            right
        };
        res.depth += 1;
        return res;
    }
}
struct Solution;
fn main() {}
