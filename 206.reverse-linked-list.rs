/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut original = List { head };
        let mut reversed = List::empty();

        while let Some(node) = original.pop() {
            reversed.push(node);
        }

        reversed.head
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

