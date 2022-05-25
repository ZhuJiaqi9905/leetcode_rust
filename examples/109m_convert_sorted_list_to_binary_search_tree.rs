use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut len = 0;
        let mut p = &head;
        while p.is_some() {
            p = &p.as_ref().unwrap().next;
            len += 1;
        }
        let mut head = head;
        return Solution::inorder_build(0, len - 1, &mut head);
    }
    fn inorder_build(
        left: i32,
        right: i32,
        mut cur: &mut Option<Box<ListNode>>, // 用cur指向当前的mid节点
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right {
            let root = Rc::new(RefCell::new(TreeNode::new(cur.as_ref().unwrap().val)));
            *cur = cur.as_mut().unwrap().next.take();
            return Some(root);
        }

        let mid = (left + right) / 2;
        let left_tree = if right == left + 1 {
            None
        } else {
            Solution::inorder_build(left, mid - 1, cur)
        };
        // 构造根节点
        let root = Rc::new(RefCell::new(TreeNode::new(cur.as_ref().unwrap().val)));
        *cur = cur.as_mut().unwrap().next.take();
        // 构造右子树
        let right_tree = Solution::inorder_build(mid + 1, right, cur);
        // 将左右子树接到根节点上
        root.borrow_mut().left = left_tree;
        root.borrow_mut().right = right_tree;
        return Some(root);
    }
}
struct Solution;
fn main() {
    let mut n1 = Box::new(ListNode::new(9));
    let mut n2 = Box::new(ListNode::new(5));
    let mut n3 = Box::new(ListNode::new(0));
    let mut n4 = Box::new(ListNode::new(-3));
    let mut n5 = Box::new(ListNode::new(-10));
    n2.next = Some(n1);
    n3.next = Some(n2);
    n4.next = Some(n3);
    n5.next = Some(n4);

    Solution::sorted_list_to_bst(Some(n5));
}
