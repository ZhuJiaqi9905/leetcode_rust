use std::cell::RefCell;
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
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        // 利用复制dummy节点来避开rust的安全检查
        let mut right = dummy.clone();
        let mut left = dummy.as_mut();
        //  双指针
        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();
        let mut the_head = None;
        mem::swap(&mut the_head, &mut dummy.next);
        the_head
    }
}
struct Solution;
fn main() {}
