/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut freqs1 = HashMap::<_, usize>::with_capacity(s.len());
        for c in s.chars() {
            *freqs1.entry(c).or_default() += 1;
        }

        let mut freqs2 = HashMap::<_, usize>::with_capacity(t.len());
        for c in t.chars() {
            *freqs2.entry(c).or_default() += 1;
        }

        freqs1 == freqs2
    }
}
// @lc code=end

