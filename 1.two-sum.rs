/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (i, j) = two_sum(&nums, target).unwrap();
        vec![i as i32, j as i32]
    }
}

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::with_capacity(nums.len());

    for (j, &y) in nums.iter().enumerate() {
        let x = target - y;
        if let Some(&i) = seen.get(&x) {
            return Some((i, j));
        }
        seen.insert(y, j);
    }

    None
}
// @lc code=end

