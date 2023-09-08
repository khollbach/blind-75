/*
 * @lc app=leetcode id=143 lang=rust
 *
 * [143] Reorder List
 */

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

// @lc code=start
impl Solution {
    /// O(n)
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let list = List { head: head.take() };
        let (low, mut high) = list.split();
        high.reverse();
        *head = List::interleave(low, high).head;
    }
}

type Link = Option<Box<ListNode>>;

struct List {
    head: Link,
}

impl List {
    fn empty() -> Self {
        Self { head: None }
    }

    fn reverse(&mut self) {
        let mut out = List::empty();
        while let Some(node) = self.pop() {
            out.push(node);
        }
        *self = out
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut next = self.head.as_deref();
        while let Some(node) = next {
            count += 1;
            next = node.next.as_deref();
        }
        count
    }

    fn split_at(mut self, i: usize) -> (Self, Self) {
        assert!(i < self.len());
        if i == 0 {
            return (List::empty(), self);
        }

        // Find node i-1
        let mut mid = self.head.as_deref_mut().unwrap();
        for _ in 0..i - 1 {
            mid = mid.next.as_deref_mut().unwrap();
        }

        // Split before node i
        let right_half = mid.next.take();
        (self, List { head: right_half })
    }

    fn split(self) -> (Self, Self) {
        let n = self.len();
        self.split_at(n / 2)
    }

    fn interleave(mut list1: Self, mut list2: Self) -> Self {
        let mut out = List::empty();
        loop {
            match (list1.pop(), list2.pop()) {
                (Some(node1), Some(node2)) => {
                    out.push(node1);
                    out.push(node2);
                },
                (Some(node), None) => out.push(node),
                (None, Some(node)) => out.push(node),
                (None, None) => break,
            }
        }
        out.reverse();
        out
    }

    fn pop(&mut self) -> Link {
        let mut old_head = self.head.take()?;
        self.head = old_head.next.take();
        Some(old_head)
    }

    fn push(&mut self, mut new_head: Box<ListNode>) {
        debug_assert!(new_head.next.is_none());
        new_head.next = self.head.take();
        self.head = Some(new_head);
    }
}
// @lc code=end

