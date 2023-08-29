/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

pub struct Solution;

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        product_except_self(&nums)
    }
}

/// O(n)
fn _product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();

    // "prefix products" -- pp[i] = product(nums[..i])
    let mut pp = Vec::with_capacity(n + 1);
    pp.push(1);
    for &x in nums {
        pp.push(pp.last().unwrap() * x);
    }

    // "suffix products" -- sp[i] = product(nums[i..])
    let mut sp = VecDeque::with_capacity(n + 1);
    sp.push_front(1);
    for &x in nums.iter().rev() {
        sp.push_front(sp[0] * x);
    }

    (0..n).map(|i| pp[i] * sp[i + 1]).collect()
}

/// O(n), O(1) 'extra' space
fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();

    // "prefix products" -- out[i] = product(nums[..i])
    let mut out = Vec::with_capacity(n);
    let mut p = 1;
    for &x in nums {
        out.push(p);
        p *= x;
    }

    // "suffix products" -- before iteration `i`, p = product(nums[i+1..])
    let mut p = 1;
    for (i, &x) in nums.iter().enumerate().rev() {
        out[i] *= p;
        p *= x;
    }

    // out[i] = product(nums[..i]) * product(nums[i+1..])
    out
}
// @lc code=end

