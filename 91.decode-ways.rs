/*
 * @lc app=leetcode id=91 lang=rust
 *
 * [91] Decode Ways
 */

struct Solution;

fn main() {
    let s = "226".to_owned();
    let ans = Solution::num_decodings(s);
    assert_eq!(ans, 3);

    let s = "10".to_owned();
    let ans = Solution::num_decodings(s);
    assert_eq!(ans, 1);
}

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        assert!(s.is_ascii());
        ways(s.as_bytes()) as i32
    }
}

fn ways(s: &[u8]) -> u32 {
    if s.is_empty() {
        return 0;
    }

    let n = s.len();
    let mut dp = vec![0; n + 1];

    dp[n] = 1;
    dp[n - 1] = is_basic_code(&s[n - 1..]) as u32;

    for i in (0..n - 1).rev() {
        if is_basic_code(&s[i..i + 1]) {
            dp[i] += dp[i + 1];
        }
        if is_basic_code(&s[i..i + 2]) {
            dp[i] += dp[i + 2];
        }
    }

    dbg!(&dp);
    dp[0]
}

fn is_basic_code(s: &[u8]) -> bool {
    match s.len() {
        1 => (b'1'..=b'9').contains(&s[0]),
        2 => match s[0] {
            b'1' => (b'0'..=b'9').contains(&s[1]),
            b'2' => (b'0'..=b'6').contains(&s[1]),
            _ => false,
        }
        _ => false,
    }
}
// @lc code=end

