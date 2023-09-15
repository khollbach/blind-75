/*
 * @lc app=leetcode id=124 lang=rust
 *
 * [124] Binary Tree Maximum Path Sum
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_path(&root.unwrap().borrow()).overall
    }
}

struct MaxPath {
    overall: i32,
    rooted: i32,
}

impl MaxPath {
    const NEG_INFTY: Self = Self {
        overall: i32::MIN,
        rooted: i32::MIN,
    };
}

/// O(n)
fn max_path(root: &TreeNode) -> MaxPath {
    let left = match root.left {
        Some(ref node) => max_path(&node.borrow()),
        None => MaxPath::NEG_INFTY,
    };
    let right = match root.right {
        Some(ref node) => max_path(&node.borrow()),
        None => MaxPath::NEG_INFTY,
    };

    let rooted = root.val + *[0, left.rooted, right.rooted].iter().max().unwrap();
    let using_root = root.val + left.rooted.clamp(0, i32::MAX) + right.rooted.clamp(0, i32::MAX);
    let overall = *[using_root, left.overall, right.overall]
        .iter()
        .max()
        .unwrap();
    MaxPath { rooted, overall }
}

// @lc code=end
