/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fib(n)
    }
}

fn fib(n: i32) -> i32 {
    assert!(n >= 0);
    let (mut a, mut b) = (1, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}
// @lc code=end

