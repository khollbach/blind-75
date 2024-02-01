/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        if nums.iter().all(|&x| x < 0) {
            return nums.into_iter().max().unwrap();
        }

        let mut overall_best = 0;

        let mut curr_sum = 0;

        for x in nums {
            curr_sum += x;
            if curr_sum < 0 {
                curr_sum = 0;
            }

            overall_best = max(overall_best, curr_sum);
        }

        overall_best
    }
}
// @lc code=end

