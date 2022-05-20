use std::cell::{Ref, RefCell};
use std::collections::HashSet;
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
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Solution::set_camera(root, false, &mut ans);
        ans
    }
    fn set_camera(root: Option<Rc<RefCell<TreeNode>>>, has_parent: bool, ans: &mut i32) -> i32 {
        if root.is_none() {
            return -1;
        }
        let root = root.unwrap();
        let left = Solution::set_camera(root.borrow().left.clone(), true, ans);
        let right = Solution::set_camera(root.borrow().right.clone(), true, ans);
        if left == -1 && right == -1 {
            if has_parent {
                // 让父节点cover自己
                return 0;
            } else {
                // 自己放摄像头
                *ans += 1;
                return 2;
            }
        }
        if left == 0 || right == 0 {
            // 左右子节点存在没有被cover的
            *ans += 1;
            return 2;
        }
        if left == 2 || right == 2 {
            return 1;
        }
        // 剩下 left == 1 && right == 1 的情况
        // 即当前节点的左右子节点都被 cover
        if has_parent {
            // 如果有父节点的话，可以等父节点 cover 自己
            return 0;
        } else {
            // 没有父节点，只能自己 set 一个摄像头
            *ans += 1;
            return 2;
        }
    }
}
struct Solution;
fn main() {}
