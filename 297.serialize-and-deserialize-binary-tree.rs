/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
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
use std::fmt::Write as _;

struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Link) -> String {
        let mut out = String::new();
        serialize(root, &mut out);
        out
    }
	
    fn deserialize(&self, data: String) -> Link {
        let mut lines = data.lines().map(|line| line.parse().ok());
        let root = deserialize(&mut lines);
        assert!(lines.next().is_none());
        root
    }
}

/// O(n)
fn serialize(root: Link, out: &mut String) {
    match root {
        None => writeln!(out, "null").unwrap(),
        Some(node) => {
            let node = node.borrow();
            writeln!(out, "{}", node.val);
            serialize(Link::clone(&node.left), out);
            serialize(Link::clone(&node.right), out);
        }
    }
}

/// O(n)
fn deserialize(lines: &mut impl Iterator<Item = Option<i32>>) -> Link {
    let val = lines.next().unwrap()?;
    let left = deserialize(lines);
    let right = deserialize(lines);
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
// @lc code=end
