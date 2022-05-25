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
// 使用堆需要实现Ord和PartialOrd两个trait
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // dummy节点是为了让引用p更方便地使用
        let mut dummy = Box::new(ListNode::new(0));
        // 大根堆
        let mut heap = BinaryHeap::new();
        for node in lists {
            if node.is_some() {
                heap.push(node.unwrap());
            }
        }
        let mut p = &mut dummy;
        while !heap.is_empty() {
            let mut node = heap.pop().unwrap();
            // 使用swap黑魔法，得到node.next，同时把node.next设置为空
            let mut next_node = None;
            mem::swap(&mut next_node, &mut node.next);
            if next_node.is_some() {
                // 如果下一个节点非空，就把它加到堆里
                heap.push(next_node.unwrap());
            }
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }
        let head = dummy.next;
        head
    }
}
struct Solution;
fn main() {}
