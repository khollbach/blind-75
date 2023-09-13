/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
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
    pub fn build_tree(pre: Vec<i32>, in_: Vec<i32>) -> Link {
        build_tree(&pre, &in_)
    }
}

/// O(n^2) in the worst case, I think... consider a linked list of all "left"s
/// 
/// Maybe there's a clever thing where you eat the list from both ends to find
/// the root, and somehow it ends up being O(n) total work after amortizing???
/// 
/// Update: you can do O(n) if you compute an elem->idx map once at the start.
fn build_tree(pre: &[i32], in_: &[i32]) -> Link {
    assert_eq!(pre.len(), in_.len());
    if pre.is_empty() {
        return None;
    }

    let root_val = pre[0];
    let (i, _) = in_.iter().enumerate().find(|&(_, &x)| x == root_val).unwrap();

    let left = build_tree(&pre[1..=i], &in_[..i]);
    let right = build_tree(&pre[i + 1..], &in_[i + 1..]);

    Some(Rc::new(RefCell::new(TreeNode {
        val: root_val,
        left,
        right,
    })))
}
// @lc code=end
