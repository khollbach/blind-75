/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            n => max(rob(&nums[1..]), rob(&nums[..n - 1])),
        }
    }
}

fn rob(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    let mut dp = vec![0; n + 1];

    // base cases
    dp[n] = 0;
    dp[n - 1] = nums[n - 1];

    for i in (0..n - 1).rev() {
        dp[i] = max(dp[i + 1], nums[i] + dp[i + 2]);
    }

    dp[0]
}
// @lc code=end

