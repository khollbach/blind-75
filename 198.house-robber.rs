/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let n = nums.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 0;
        dp[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            dp[i] = max(dp[i + 1], nums[i] + dp[i + 2]);
        }

        dp[0]
    }
}
// @lc code=end

