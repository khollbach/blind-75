/*
 * @lc app=leetcode id=152 lang=rust
 *
 * [152] Maximum Product Subarray
 */

// bf: n^3
// prefix/sufix products (num_zeros,nonzero_product): n^2
// greedy-ish left-to-right thing; O(n)
//  (ie linear dp, with space optimization)
// ^ the interesting thing about this is the need to compute both
//   the most negative and most positive answer for every subproblem (prefix)
//   due to there being minus signs around. I've not seen anything quite like it.
// note: there's also an even cleverer O(n) thing where you do a pass from the
// left and a pass from the right -- the reasoning is subtle (see lc
// discussion).

// @lc code=start
use std::cmp::{max, min};
use std::mem::swap;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());

        // Edge-case: no positive numbers, and no adjacent negative numbers --
        // meaning any negative numbers are interspersed with zeros. This is the
        // only way we end unable to get an answer of >= 1.
        if nums.iter().all(|&x| x <= 0) && nums.windows(2).all(|xy| !(xy[0] < 0 && xy[1] < 0)) {
            return nums.into_iter().max().unwrap();
        }

        let mut overall_best = 1;

        let mut most_pos = 1;
        let mut most_neg = 1;

        for x in nums {
            debug_assert!(most_pos >= 1);
            debug_assert!(most_neg <= 1);

            if x == 0 {
                most_pos = 1;
                most_neg = 1;
            } else {
                most_pos *= x;
                most_neg *= x;
                if x < 0 {
                    swap(&mut most_pos, &mut most_neg);
                }
                most_pos = max(most_pos, 1);
                most_neg = min(most_neg, 1);
            }

            overall_best = max(overall_best, most_pos);
        }

        overall_best
    }
}
// @lc code=end

