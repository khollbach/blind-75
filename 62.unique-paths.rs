/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
use std::convert::{TryFrom, TryInto};

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        assert!(m >= 1);
        assert!(n >= 1);

        let i = m - 1;
        let j = n - 1;
        choose(i + j, i)
    }
}

fn choose(a: i32, b: i32) -> i32 {
    let a = u128::try_from(a).unwrap();
    let b = u128::try_from(b).unwrap();
    assert!(b <= a);
    let c = a - b;

    let (b, c) = sort(b, c); // wlog, b < c
    let out = (c+1..=a).product::<u128>() / factorial(b);
    out.try_into().unwrap()
}

fn sort<T: Ord>(x: T, y: T) -> (T, T) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn factorial(a: u128) -> u128 {
    (1..=a).product()
}
// @lc code=end

