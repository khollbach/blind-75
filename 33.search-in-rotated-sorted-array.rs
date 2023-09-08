/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    /// O(lg n)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // uniq? O(n) time check
        debug_assert_eq!(nums.iter().collect::<HashSet<_>>().len(), nums.len());

        search(&nums, target).map(|idx| idx as i32).unwrap_or(-1)
    }
}

fn search(nums: &[i32], t: i32) -> Option<usize> {
    let n = nums.len();

    let mut i = 0;
    let mut j = n - 1; // inclusive

    while i <= j {
        let mid = (i + j) / 2;

        if nums[mid] == t {
            return Some(mid);
        } else if nums[i] <= t && t <= nums[mid] {
            // easy case
            j = mid - 1;
        } else if nums[mid] <= t && t <= nums[j] {
            // easy case
            i = mid + 1;
        } else if nums[i] <= nums[mid] {
            // look on the other side, where there's a dip
            i = mid + 1;
        } else if nums[mid] <= nums[j] {
            // look on the other side, where there's a dip
            j = mid - 1;
        } else {
            panic!("input array not sorted");
        }
    }

    None
}
// @lc code=end

