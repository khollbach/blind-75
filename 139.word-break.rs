/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */

// @lc code=start
use std::collections::HashSet;

// 0123456789=n
// oneormore_
// 1  1  1001 (dp)
// ^      (i)

// "" n=0

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let words: HashSet<_> = word_dict.into_iter().collect();

        let mut dp = vec![false; n + 1];
        dp[n] = true;
        for i in (0..n).rev() {
            dp[i] = (i+1..=n).any(|j| dp[j] && words.contains(&s[i..j]));
        }
        dp[0]
    }
}
// @lc code=end

