/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;

const ALPHABET_SIZE: usize = 26;

impl Solution {
    /// O(len(s) + len(t))
    pub fn is_anagram(s: String, t: String) -> bool {
        freqs(&s) == freqs(&t)
    }
}

/// O(n)
fn freqs(s: &str) -> HashMap<char, usize> {
    let mut freqs = HashMap::with_capacity(ALPHABET_SIZE);
    for c in s.chars() {
        *freqs.entry(c).or_default() += 1;
    }
    freqs
}
// @lc code=end

