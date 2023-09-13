/*
 * @lc app=leetcode id=235 lang=rust
 *
 * [235] Lowest Common Ancestor of a Binary Search Tree
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
use std::cmp::Ordering::{Less, Equal, Greater};

impl Solution {
    /// O(depth)
    pub fn lowest_common_ancestor(root: Link, p: Link, q: Link) -> Link {
        let p_val = p?.borrow().val;
        let q_val = q?.borrow().val;

        let mut curr = root?;
        loop {
            let curr_val = curr.borrow().val;

            match (p_val.cmp(&curr_val), q_val.cmp(&curr_val)) {
                (Less, Less) => {
                    let tmp = curr.borrow().left.clone()?;
                    curr = tmp;
                }
                (Greater, Greater) => {
                    let tmp = curr.borrow().right.clone()?;
                    curr = tmp;
                }
                (Equal, Equal) | _ => return Some(curr),
            }
        }
    }
}
// @lc code=end
