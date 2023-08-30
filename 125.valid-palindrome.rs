/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

pub struct Solution;

// @lc code=start
impl Solution {
    /// O(n)
    pub fn is_palindrome(s: String) -> bool {
        two_pointer(&s)
    }
}

/// O(n)
pub fn is_palindrome(s: String) -> bool {
    let normalized: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // SAFETY: normalized contains only ASCII bytes.
    let reversed = unsafe {
        let mut tmp = normalized.clone();
        tmp.as_bytes_mut().reverse();
        tmp
    };

    normalized == reversed
}

/// O(n)
fn two_pointer(s: &str) -> bool {
    let n = s.len();
    let bytes = s.as_bytes();

    let mut i = 0;
    let mut j = s.len() as isize - 1;

    loop {
        // Skip non-alphanumeric characters.
        while i < n && !bytes[i].is_ascii_alphanumeric() {
            i += 1;
        }
        while j >= 0 && !bytes[j as usize].is_ascii_alphanumeric() {
            j -= 1;
        }

        // If the pointers cross, we win.
        if i as isize >= j {
            return true;
        }

        if bytes[i].to_ascii_lowercase() == bytes[j as usize].to_ascii_lowercase() {
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }
}
// @lc code=end
