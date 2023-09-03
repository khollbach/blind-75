/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

pub struct Solution;

// @lc code=start
use std::cmp::{min, max};

impl Solution {
    /// O(n)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;
        let mut curr_min = prices[0];

        for &x in &prices[1..] {
            let profit = x - curr_min;
            max_profit = max(max_profit, profit);

            curr_min = min(curr_min, x);
        }

        max_profit
    }
}
// @lc code=end

