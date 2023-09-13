/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
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
    /// O(n^2)
    pub fn is_subtree(root: Link, sub_root: Link) -> bool {
        match (root, sub_root) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some(r), Some(s)) => {
                let left = r.borrow().left.clone();
                let right = r.borrow().right.clone();

                r == Rc::clone(&s)
                    || Self::is_subtree(left, Some(Rc::clone(&s)))
                    || Self::is_subtree(right, Some(s)) 
            }
        }
    }
}
// @lc code=end
