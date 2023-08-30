/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        longest_consecutive(nums)
    }
}

use std::collections::HashSet;

/// O(n), simplified implementation
fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums: HashSet<_> = nums.into_iter().collect();

    nums.iter().map(|&x| {
        if nums.contains(&(x - 1)) {
            // Ignore x, since it's not the beginning of a sequence.
            return 0;
        }

        // Count the length of the sequence starting at x.
        for y in x.. {
            if !nums.contains(&y) {
                return y - x;
            }
        }
        unreachable!()
    }).max().unwrap_or(0)
}

use std::collections::HashMap;

/// Information about the longest consecutive sequence starting from a given
/// element.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Info {
    Known {
        /// Strictly positive.
        length: i32,
    },
    Unknown,
    Unimportant,
}

/// O(n)
pub fn longest_consecutive_(nums: Vec<i32>) -> i32 {
    let mut infos = HashMap::with_capacity(nums.len());
    for &x in &nums {
        infos.insert(x, Info::Unknown);
    }

    for x in nums {
        // Avoid accidental n^2 runtime.
        if infos.get(&x) == Some(&Info::Unimportant) {
            continue;
        }

        // Find the value of `y` such that:
        // * `x..y` is a subset of nums.
        // * `x..=y` is not a subset nums.
        let mut y = x;

        // If y not in nums, then we're done looking.
        while let Some(info) = infos.get_mut(&y) {
            match *info {
                // Mark y as shadowed by x; keep looking.
                Info::Unknown => *info = Info::Unimportant,

                // ok; we can infer the answer.
                Info::Known { length } => {
                    y += length;
                    break;
                }

                // (Unimportant implies shadowed by something known.)
                Info::Unimportant => unreachable!(),
            }
            y += 1;
        }

        let length = y - x;
        infos.insert(x, Info::Known { length });
    }

    // Find the max.
    infos.values().filter_map(|&info| match info {
        Info::Known { length } => Some(length),
        Info::Unimportant => None,
        Info::Unknown => unreachable!(),
    }).max().unwrap_or(0)
}
// @lc code=end

