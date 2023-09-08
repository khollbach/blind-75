/*
 * @lc app=leetcode id=153 lang=rust
 *
 * [153] Find Minimum in Rotated Sorted Array
 */

pub struct Solution;

// @lc code=start
use std::{collections::HashSet, cmp::min};

impl Solution {
    /// O(lg n)
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // uniq? O(n) time check
        debug_assert_eq!(nums.iter().collect::<HashSet<_>>().len(), nums.len());

        find_min(&nums)
    }
}

fn find_min(nums: &[i32]) -> i32 {
    assert!(!nums.is_empty());
    let n = nums.len();

    // base cases
    match n {
        1 => return nums[0],
        2 => return min(nums[0], nums[1]),
        _ => (),
    }

    let mid = n / 2;
    if nums[0] < nums[mid] && nums[mid] < nums[n - 1] {
        // edge-case: array is exactly sorted
        nums[0]
    } else if nums[0] < nums[mid] {
        // throw out nums[0..=mid]
        find_min(&nums[mid + 1..])
    } else if nums[mid] < nums[n - 1] {
        // throw out nums[mid + 1..]
        find_min(&nums[..=mid])
    } else {
        panic!("impossible; array wasn't sorted");
    }
}

// @lc code=end

