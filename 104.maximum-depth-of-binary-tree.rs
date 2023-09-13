/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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
use std::cmp::max;

impl Solution {
    /// O(n)
    pub fn max_depth(root: Link) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                1 + max(Self::max_depth(left), Self::max_depth(right))
            }
        }
    }
}
// @lc code=end

