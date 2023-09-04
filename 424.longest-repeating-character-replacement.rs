/*
 * @lc app=leetcode id=424 lang=rust
 *
 * [424] Longest Repeating Character Replacement
 */

pub struct Solution;

// @lc code=start
use std::cmp::max;

impl Solution {
    /// O(n)
    pub fn character_replacement(s: String, k: i32) -> i32 {
        assert!(k >= 0);
        assert!(s.chars().all(|c| c.is_ascii_uppercase()));
        let s = s.as_bytes();

        let mut best = 0;

        for target in b'A'..=b'Z' {
            // The start of the sliding window.
            let mut i = 0;

            // Number of non-`target` chars in the current window.
            let mut num_bad = 0;

            for j in 0..s.len() {
                // Push s[j] to the end of the window.

                // Ensure the constraint is met; pop from the front, if needed.
                if s[j] != target {
                    if num_bad < k {
                        num_bad += 1;
                    } else {
                        // Find and then pop the first bad char.
                        while s[i] == target {
                            i += 1;
                        }
                        i += 1;
                    }
                }

                // Window is s[i..=j].
                let window_len = j + 1 - i;
                best = max(best, window_len);
            }
        }

        best as i32
    }
}
// @lc code=end

