/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
use std::collections::HashMap;

struct Trie {
    nodes: Vec<Node>,
    root: usize,
}

#[derive(Default)]
struct Node {
    children: HashMap<char, usize>,
    word_ends_here: bool,
}

impl Trie {
    fn new() -> Self {
        Self { nodes: vec![Node::default()], root: 0 }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = self.root;
        for c in word.chars() {
            curr = match self.nodes[curr].children.get(&c) {
                Some(&idx) => idx,
                None => {
                    let new_idx = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[curr].children.insert(c, new_idx);
                    new_idx
                }
            }
        }
        self.nodes[curr].word_ends_here = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut curr = self.root;
        for c in word.chars() {
            let Some(&next) = self.nodes[curr].children.get(&c) else {
                return false;
            };
            curr = next;
        }
        self.nodes[curr].word_ends_here
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self.root;
        for c in prefix.chars() {
            let Some(&next) = self.nodes[curr].children.get(&c) else {
                return false;
            };
            curr = next;
        }
        true
    }
}

// @lc code=end
