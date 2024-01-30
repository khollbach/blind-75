/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// idea; for each starting point, expand outwards as far as you can

// @lc code=start
use std::convert::TryFrom;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        assert!(s.is_ascii());
        let s = s.as_bytes();
        let n = s.len();

        let odd = (0..n).map(|i| odd(s, i)).max_by_key(|s| s.len()).unwrap_or(&[]);
        let even = (0..n - 1).map(|i| even(s, i)).max_by_key(|s| s.len()).unwrap_or(&[]);
        let best = if odd.len() > even.len() { odd } else { even };
        String::from_utf8(best.to_vec()).unwrap()
    }
}

fn even(s: &[u8], i: usize) -> &[u8] {
    let n = s.len();
    assert!(i + 1 < n);

    let mut a = i as isize;
    let mut b = i + 1;
    while 0 <= a && b < n {
        if s[a as usize] != s[b] {
            break;
        }
        a -= 1;
        b += 1;
    }
    a += 1;
    b -= 1;

    &s[a as usize..=b] // possibly empty
}

fn odd(s: &[u8], i: usize) -> &[u8] {
    let n = s.len();
    assert!(i < n);

    let mut a = i as isize;
    let mut b = i;
    while 0 <= a && b < n {
        if s[a as usize] != s[b] {
            break;
        }
        a -= 1;
        b += 1;
    }
    a += 1;
    b -= 1;

    &s[a as usize..=b]
}
// @lc code=end
