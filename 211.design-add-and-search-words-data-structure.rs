/*
 * @lc app=leetcode id=211 lang=rust
 *
 * [211] Design Add and Search Words Data Structure
 */

// @lc code=start
use std::collections::HashMap;

struct WordDictionary {
    nodes: Vec<Node>,
    root: usize,
}

#[derive(Default)]
struct Node {
    children: HashMap<char, usize>,
    word_ends_here: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self { nodes: vec![Node::default()], root: 0 }
    }

    /// word must not contain '.'s.
    fn add_word(&mut self, word: String) {
        let mut curr = self.root;
        for c in word.chars() {
            assert_ne!(c, '.');
            curr = match self.nodes[curr].children.get(&c) {
                Some(&idx) => idx,
                None => {
                    let new_idx = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[curr].children.insert(c, new_idx);
                    new_idx
                }
            };
        }
        self.nodes[curr].word_ends_here = true;
    }

    /// Treat the '.' character as a wildcard -- matches any letter.
    fn search(&self, word: String) -> bool {
        self.dfs(self.root, &word)
    }

    /// Helper for `search`.
    fn dfs(&self, curr: usize, word: &str) -> bool {
        if word.is_empty() {
            return self.nodes[curr].word_ends_here;
        }

        let (c, rest) = take_first(word).unwrap();

        if c != '.' {
            let Some(&next) = self.nodes[curr].children.get(&c) else {
                return false;
            };
            self.dfs(next, rest)
        } else {
            self.nodes[curr].children.values().any(|&next| {
                self.dfs(next, rest)
            })
        }
    }
}

fn take_first(s: &str) -> Option<(char, &str)> {
    let c = s.chars().next()?;
    let idx = s.char_indices().map(|(i, _c)| i).nth(1).unwrap_or(s.len());
    Some((c, &s[idx..]))
}

// @lc code=end

