/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 */

/*
 bear_
b32210
a22210 
k22110
e22110
r11110
_00000

*/

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s = text1.as_bytes();
        let t = text2.as_bytes();

        // dp[i][j] is the solution to the subproblem (s[i..], t[j..])
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

        // base cases:
        // dp[s.len()][j] = 0
        // dp[i][t.len()] = 0

        for i in (0..s.len()).rev() {
            for j in (0..t.len()).rev() {
                let skip_s = dp[i + 1][j];
                let skip_t = dp[i][j + 1];
                let use_match = if s[i] == t[j] {
                    1 + dp[i + 1][j + 1]
                } else {
                    0
                };

                dp[i][j] = [skip_s, skip_t, use_match].iter().copied().max().unwrap();
            }
        }

        dp[0][0]
    }
}
// @lc code=end

