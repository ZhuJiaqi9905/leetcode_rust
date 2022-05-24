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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut p = &mut dummy;
        while list1.is_some() && list2.is_some() {
            let l1 = &mut list1;
            let l2 = &mut list2;
            let l = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                l1
            } else {
                l2
            };
            mem::swap(p, l);
            mem::swap(l, &mut p.as_mut().unwrap().next);
            p = &mut p.as_mut().unwrap().next;
        }
        mem::swap(
            p,
            if list1.is_none() {
                &mut list2
            } else {
                &mut list1
            },
        );
        dummy
    }
}
struct Solution;
fn main() {}
