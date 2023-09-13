/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
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
    pub fn kth_smallest(root: Link, k: i32) -> i32 {
        assert!(k >= 1);
        let k = k as usize - 1; // 0-based

        kth_smallest(&root.unwrap().borrow(), &mut 0, k).unwrap()
    }
}

/// O(n)
fn kth_smallest(node: &TreeNode, i: &mut usize, k: usize) -> Option<i32> {
    if let Some(l) = node.left.as_ref() {
        if let Some(ans) = kth_smallest(&l.borrow(), i, k) {
            return Some(ans);
        }
    }

    if *i == k {
        return Some(node.val);
    }
    *i += 1;

    if let Some(r) = node.right.as_ref() {
        if let Some(ans) = kth_smallest(&r.borrow(), i, k) {
            return Some(ans);
        }
    }

    None
}
// @lc code=end

