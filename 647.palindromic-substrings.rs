/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        assert!(s.is_ascii());
        let s = s.as_bytes();
        let n = s.len();

        let mut count = 0u32;
        for i in 0..n {
            count += odd(s, i);
        }
        for i in 0..n - 1 {
            count += even(s, i);
        }
        count as i32
    }
}

fn odd(s: &[u8], i: usize) -> u32 {
    let n = s.len();
    assert!(i < n);

    let mut count = 0;

    let mut a = i as isize;
    let mut b = i;
    while 0 <= a && b < n {
        if s[a as usize] != s[b] {
            break;
        }

        count += 1;

        a -= 1;
        b += 1;
    }

    count
}

fn even(s: &[u8], i: usize) -> u32 {
    let n = s.len();
    assert!(i + 1 < n);

    let mut count = 0;

    let mut a = i as isize;
    let mut b = i + 1;
    while 0 <= a && b < n {
        if s[a as usize] != s[b] {
            break;
        }

        count += 1;

        a -= 1;
        b += 1;
    }

    count
}
// @lc code=end
