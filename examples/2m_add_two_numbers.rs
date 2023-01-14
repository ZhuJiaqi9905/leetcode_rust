// 单向链表的操作，要注意到把Box<T>和&mut Box<T>搭配使用，
// 实现在Rust的编译检查条件下仍能够正确编程。

// Definition for singly-linked list.

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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        let mut carry = 0;
        let mut val: i32;
        // 用一个头结点
        let mut head = Some(Box::new(ListNode::new(0)));

        // p_node是在链表中移动的指针，所以用引用来实现
        let mut p_node = &mut head;
        let mut begin = true;
        while l1.is_some() || l2.is_some() {
            let l1_val = match &l1 {
                Some(node) => {
                    let l1_val = node.val;
                    l1 = l1.unwrap().next;
                    l1_val
                }
                None => 0,
            };
            let l2_val = match &l2 {
                Some(node) => {
                    let l2_val = node.val;
                    l2 = l2.unwrap().next;
                    l2_val
                }
                None => 0,
            };
            val = l1_val + l2_val + carry;
            carry = val / 10;
            if begin {
                p_node.as_mut().unwrap().val = val % 10;
                begin = false;
            } else {
                p_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(val % 10)));
                p_node = &mut p_node.as_mut().unwrap().next;
            }
        }
        if carry != 0 {
            p_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        return head;
    }
}
pub struct Solution;
pub fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let mut res = Solution::add_two_numbers(l1, l2);
    while res.is_some() {
        print!("{}", res.as_ref().unwrap().val);
        res = res.unwrap().next;
    }
    println!("");
}
