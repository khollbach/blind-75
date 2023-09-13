/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    /// O(n)
    pub fn is_same_tree(p: Link, q: Link) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(p), Some(q)) => {
                let p_val = p.borrow().val;
                let pl = p.borrow().left.clone();
                let pr = p.borrow().right.clone();

                let q_val = q.borrow().val;
                let ql = q.borrow().left.clone();
                let qr = q.borrow().right.clone();

                p_val == q_val && Self::is_same_tree(pl, ql) && Self::is_same_tree(pr, qr)
            }
        }
    }
}
// @lc code=end
