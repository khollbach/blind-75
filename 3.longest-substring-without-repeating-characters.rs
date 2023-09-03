/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

pub struct Solution;

// @lc code=start
use std::{
    collections::HashSet,
    cmp::max,
};

impl Solution {
    /// O(n)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut best = 0;

        // We use `chars` like a pointer into s.
        // `chars.next()` fetches & increments the head of the window.
        let mut chars = s.chars();
        let mut window = HashSet::new();

        for c in s.chars() {
            // Enforce uniqueness by shrinking the window from the left.
            while window.contains(&c) {
                window.remove(&chars.next().unwrap());
            }
            window.insert(c);

            best = max(best, window.len());
        }

        best as i32
    }
}
// @lc code=end

