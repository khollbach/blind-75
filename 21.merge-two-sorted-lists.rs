/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut out = List::empty();

        let mut list1 = List { head: list1 };
        let mut list2 = List { head: list2 };
        loop {
            match (list1.peek(), list2.peek()) {
                (Some(x), Some(y)) if x <= y => out.push(list1.pop().unwrap()),
                (Some(_), Some(_)) => out.push(list2.pop().unwrap()),
                (Some(_), None) => out.push(list1.pop().unwrap()),
                (None, Some(_)) => out.push(list2.pop().unwrap()),
                (None, None) => break,
            }
        }

        out.reverse();
        out.head
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

    fn peek(&self) -> Option<i32> {
        Some(self.head.as_ref()?.val)
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

