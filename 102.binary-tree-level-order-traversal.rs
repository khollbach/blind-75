/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 */

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Link,
    pub right: Link,
}

// @lc code=start
type Link = Option<Rc<RefCell<TreeNode>>>;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    /// O(n)
    pub fn level_order(root: Link) -> Vec<Vec<i32>> {
        let mut out = vec![];

        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back((0, node));
        }

        while let Some((level, node)) = q.pop_front() {
            if level >= out.len() {
                out.resize(level + 1, vec![]);
            }
            out[level].push(node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                q.push_back((level + 1, left));
            }
            if let Some(right) = node.borrow().right.clone() {
                q.push_back((level + 1, right));
            }
        }

        out
    }
}
// @lc code=end

