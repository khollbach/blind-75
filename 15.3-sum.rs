/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        two_pointer(nums)
    }
}

/// O(n^3) -- TLE
fn _three_sum_naive(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = HashSet::new();

    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut triple = vec![nums[i], nums[j], nums[k]];
                    triple.sort_unstable();
                    out.insert(triple);
                }
            }
        }
    }

    out.into_iter().collect()
}

/// O(n^2)
fn _three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();

    let mut out = HashSet::new();
    let mut seen = HashSet::with_capacity(n);

    for j in 0..n {
        for k in j + 1..n {
            let y = nums[j];
            let z = nums[k];
            let x = -y - z;
            if seen.contains(&x) {
                let mut triple = vec![x, y, z];
                triple.sort_unstable();
                out.insert(triple);
            }
        }

        seen.insert(nums[j]);
    }

    out.into_iter().collect()
}

/// O(n^2)
fn two_pointer(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut out = HashSet::new();
    for i in 0..nums.len() {
        two_sum(nums[i], &nums[i + 1..], &mut out);
    }
    out.into_iter().collect()
}

use std::cmp::Ordering::*;

/// Helper for two_pointer.
/// O(n)
fn two_sum(x: i32, nums: &[i32], out: &mut HashSet<Vec<i32>>) {
    if nums.is_empty() {
        return;
    }

    let mut j = 0;
    let mut k = nums.len() - 1;
    while j < k {
        let y = nums[j];
        let z = nums[k];

        match (x + y + z).cmp(&0) {
            Less => j += 1,
            Greater => k -= 1,
            Equal => {
                let mut triple = vec![x, y, z];
                triple.sort_unstable();
                out.insert(triple);
                j += 1;
                k -= 1;
            }
        }
    }
}
// @lc code=end

