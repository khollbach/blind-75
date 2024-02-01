/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// Note: this can be optimized by remembering the left-most winning index, as you go.
// Then the recurive case is just an O(1) comparison, and the runtime goes to O(n).

// @lc code=start
use std::convert::TryInto;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        assert!(!nums.is_empty());
        let n = nums.len();
        let mut ans = vec![false; n];
        ans[n - 1] = true; // base case
        for i in (0..n - 1).rev() {
            let num_edges: usize = nums[i].try_into().unwrap();
            ans[i] = (i + 1..=i + num_edges).any(|j| ans[j]);
        }
        ans[0]
    }
}
// @lc code=end

