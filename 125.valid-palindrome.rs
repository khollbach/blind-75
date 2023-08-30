/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

pub struct Solution;

// @lc code=start
impl Solution {
    /// O(n)
    pub fn is_palindrome(s: String) -> bool {
        let normalized: String = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        // SAFETY: normalized contains only ASCII bytes.
        let reversed = unsafe {
            let mut tmp = normalized.clone();
            tmp.as_bytes_mut().reverse();
            tmp
        };

        normalized == reversed
    }
}
// @lc code=end
