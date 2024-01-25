/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

struct Solution;

fn main() {
    let nums = vec![2, 3, 6, 7];
    let target = 7;
    let ans = combination_sum(nums, target);
    dbg!(ans);

    let expected = vec![vec![2, 2, 3], vec![7]];
    dbg!(expected);
}

fn debug_dp(dp: &Vec<Vec<usize>>) {
    for row in dp {
        for col in row {
            eprint!("{col} ");
        }
        eprintln!();
    }
}

// @lc code=start
use std::convert::TryFrom;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = usize::try_from(target).unwrap();

        let mut out = vec![];
        bt(&candidates, target, &mut vec![], &mut out);
        out
    }
}

fn bt(nums: &[i32], t: usize, curr: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if t == 0 {
        out.push(curr.clone());
        return;
    }

    if nums.is_empty() {
        return;
    }

    // Use it.
    let x = usize::try_from(nums[0]).unwrap();
    if t >= x {
        curr.push(x as i32);
        bt(nums, t - x, curr, out);
        curr.pop();
    }

    // Don't.
    bt(&nums[1..], t, curr, out);
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let target = usize::try_from(target).unwrap();
    let dp = dp(&candidates, target);

    // debug_dp(&dp);

    let mut out = vec![];
    recover_soln(&candidates, &dp, target, candidates.len(), &mut out);
    out
}

fn recover_soln(
    nums: &[i32],
    dp: &Vec<Vec<usize>>,
    t: usize,
    i: usize,
    out: &mut Vec<Vec<i32>>,
) {
    // base cases
    if (t, i) == (0, 0) {
        out.push(vec![]);
        return;
    }
    if i == 0 {
        return;
    }

    // Use it.
    let x = usize::try_from(nums[i-1]).unwrap();
    if t >= x {
        let prev_len = out.len();
        recover_soln(nums, dp, t - x, i, out);
        let extra = out.len() - prev_len;

        for o in out.iter_mut().rev().take(extra) {
            o.push(x as i32);
        }
    }

    // Don't.
    recover_soln(nums, dp, t, i - 1, out);
}

fn dp(nums: &[i32], target: usize) -> Vec<Vec<usize>> {
    let n = nums.len();

    // indexed as ans[t][i]
    let mut ans = vec![vec![0; n + 1]; target + 1];

    // Left-most column are the base-cases; ans[t][0] = 0 for t > 0.
    ans[0][0] = 1;

    for t in 0..=target {
        for i in 1..=n {
            let x = usize::try_from(nums[i-1]).unwrap();

            // Use it.
            if t >= x {
                ans[t][i] += ans[t - x][i];
            }

            // Don't.
            ans[t][i] += ans[t][i - 1];
        }
    }

    ans
}

// @lc code=end

