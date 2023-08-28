/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        for x in nums {
            if seen.contains(&x) {
                return true;
            }
            seen.insert(x);
        }
        false
    }
}
// @lc code=end

