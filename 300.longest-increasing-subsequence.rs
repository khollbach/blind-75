/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

// Note: there's an (n lg n) thing for this that's pretty sick.
// Idk if I'd be able to remember it / derive it in an interview though.

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        for i in 0..n {
            let best = (0..i)
                .filter(|&j| nums[j] < nums[i])
                .map(|j| dp[j])
                .max()
                .unwrap_or(0);
            dp[i] = 1 + best;
        }
        dp.into_iter().max().unwrap_or(0)
    }
}
// @lc code=end

