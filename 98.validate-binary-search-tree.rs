/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
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

impl Solution {
    pub fn is_valid_bst(root: Link) -> bool {
        is_valid(root, i64::MIN, i64::MAX)
    }
}

/// O(n)
fn is_valid(root: Link, min: i64, max: i64) -> bool {
    let node = match root {
        Some(r) => r,
        None => return true,
    };

    let x = node.borrow().val as i64;
    let left = node.borrow().left.clone();
    let right = node.borrow().right.clone();

    min < x && x < max && is_valid(left, min, x) && is_valid(right, x, max)
}
// @lc code=end

