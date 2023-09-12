/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 */

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Link = Option<Box<ListNode>>;

impl Solution {
    /// O(n lg k)
    pub fn merge_k_lists(lists: Vec<Link>) -> Link {
        let mut lists: Vec<_> = lists.into_iter().map(|head| List { head }).collect();

        let mut q = BinaryHeap::with_capacity(lists.len());
        for (i, l) in lists.iter().enumerate() {
            if let Some(x) = l.peek() {
                q.push((Reverse(x), i)); // min-heap
            }
        }

        let mut out = List::empty();

        while let Some((_, i)) = q.pop() {
            let node = lists[i].pop().unwrap();
            out.push(node);

            if let Some(x) = lists[i].peek() {
                q.push((Reverse(x), i));
            }
        }

        out.reverse();
        out.head
    }
}

struct List {
    head: Link,
}

impl List {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, mut node: Box<ListNode>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    fn pop(&mut self) -> Link {
        let mut node = self.head.take()?;
        self.head = node.next.take();
        Some(node)
    }

    fn peek(&self) -> Option<i32> {
        Some(self.head.as_ref()?.val)
    }

    fn reverse(&mut self) {
        let mut out = Self::empty();
        while let Some(node) = self.pop() {
            out.push(node);
        }
        *self = out;
    }
}
// @lc code=end
