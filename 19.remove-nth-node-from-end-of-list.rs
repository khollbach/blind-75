/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, i: i32) -> Option<Box<ListNode>> {
        let mut list = List { head };
        assert!(i >= 0);
        let i = i as usize;

        let n = list.len();
        assert_ne!(i, 0);
        assert!(i <= n);

        list.remove(n - i);
        list.head
    }
}

type Link = Option<Box<ListNode>>;

struct List {
    head: Link,
}

impl List {
    fn len(&self) -> usize {
        let mut count = 0;
        let mut next = self.head.as_deref();
        while let Some(node) = next {
            count += 1;
            next = node.next.as_deref();
        }
        count
    }

    fn remove(&mut self, i: usize) {
        assert!(i < self.len());
        if i == 0 {
            let mut node = self.head.take().unwrap();
            self.head = node.next.take();
            return;
        }

        // Find node i-1
        let mut ptr = self.head.as_deref_mut().unwrap();
        for _ in 0..i - 1 {
            ptr = ptr.next.as_deref_mut().unwrap();
        }

        let mut target_node = ptr.next.take().unwrap();
        ptr.next = target_node.next.take();
    }
}
// @lc code=end

