/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
use std::cmp::min;
use std::convert::{TryInto, TryFrom};

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        assert!(coins.iter().all(|&c| c >= 0));

        let ans = min_coins(&coins, amount);
        if ans == u32::MAX {
            -1
        } else {
            ans.try_into().unwrap()
        }
    }
}

fn min_coins(coins: &[i32], amount: i32) -> u32 {
    let amount = usize::try_from(amount).unwrap();
    let n = coins.len();
    
    // dp[a][i] is the solution to min_coins(&coins[i..], a)
    let mut dp = vec![vec![0; n + 1]; amount + 1];

    // base cases
    for a in 1..=amount {
        dp[a][n] = u32::MAX;
    }
    for i in 0..=n {
        dp[0][i] = 0;
    }

    for a in 1..=amount {
        for i in (0..n).rev() {
            let c: usize = coins[i].try_into().unwrap();
            let use_it = if c <= a { dp[a - c][i].saturating_add(1) } else { u32::MAX };
            let dont = dp[a][i + 1];
            dp[a][i] = min(use_it, dont);
        }
    }

    dp[amount][0]
}
// @lc code=end
