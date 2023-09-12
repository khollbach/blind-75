/*
 * @lc app=leetcode id=226 lang=rust
 *
 * [226] Invert Binary Tree
 */

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Link,
    pub right: Link,
}

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// O(n)
    pub fn invert_tree(root: Link) -> Link {
        let node = root?;

        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        node.borrow_mut().left = right;
        node.borrow_mut().right = left;

        Self::invert_tree(node.borrow().left.clone());
        Self::invert_tree(node.borrow().right.clone());

        Some(node)
    }
}
// @lc code=end
