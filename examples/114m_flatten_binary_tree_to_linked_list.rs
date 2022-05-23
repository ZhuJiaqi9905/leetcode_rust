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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let root = root.as_ref().unwrap();
        // 把左子树拉成链表
        Solution::flatten(&mut root.borrow().left.clone());
        // 把右子树拉成链表
        Solution::flatten(&mut root.borrow().right.clone());
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        root.borrow_mut().left = None;
        root.borrow_mut().right = left;
        // 沿着右子树的链往下走
        let mut node = root.clone();
        while node.borrow().right.is_some() {
            let r = node.borrow().right.clone();
            node = r.unwrap();
        }
        node.borrow_mut().right = right;
    }
}
struct Solution;
fn main() {}
