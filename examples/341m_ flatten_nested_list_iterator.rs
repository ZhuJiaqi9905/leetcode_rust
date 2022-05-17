use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    nested: Vec<i32>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut nested = vec![];
        for n in &nestedList {
            NestedIterator::dfs(n, &mut nested);
        }
        Self { nested, index: 0 }
    }
    fn dfs(integer: &NestedInteger, nested: &mut Vec<i32>) {
        match integer {
            NestedInteger::Int(val) => nested.push(*val),
            NestedInteger::List(l) => {
                for n in l {
                    NestedIterator::dfs(n, nested);
                }
            }
        }
    }
    fn next(&mut self) -> i32 {
        let ans = self.nested[self.index];
        self.index += 1;
        ans
    }

    fn has_next(&self) -> bool {
        self.index < self.nested.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
struct Solution;
fn main() {}
