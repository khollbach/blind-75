/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 */

pub struct Solution;

// @lc code=start
use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    /// O(n)
    pub fn min_window(s: String, t: String) -> String {
        assert!(s.is_ascii());
        assert!(t.is_ascii());
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut best = b"".as_slice(); // default retval, if no soln found
        let mut best_len = usize::MAX; // +infty
        
        // Char freqs of the current window; represented so that we can quickly
        // check if the current window is a candidate solution.
        let mut deficit = HashMap::<_, usize>::new();
        let mut surplus = HashMap::<_, usize>::new();
        for &c in t {
            *deficit.entry(c).or_default() += 1;
        }

        // Curr window start.
        let mut i = 0;

        for j in 0..s.len() {
            // Push s[j] to the back.
            if let Entry::Occupied(mut f) = deficit.entry(s[j]) {
                *f.get_mut() -= 1;
                if *f.get() == 0 {
                    f.remove_entry();
                }
            } else {
                *surplus.entry(s[j]).or_default() += 1;
            }

            // Pop any surplus from the front.
            while i < s.len() && surplus.contains_key(&s[i]) {
                let f = surplus.get_mut(&s[i]).unwrap();
                *f -= 1;
                if *f == 0 {
                    surplus.remove(&s[i]);
                }

                i += 1;
            }

            // Consider s[i..=j]
            if deficit.is_empty() && j + 1 - i < best_len {
                best = &s[i..=j];
                best_len = best.len();
            }
        }

        String::from_utf8(best.to_vec()).expect("all chars should be ASCII")
    }
}
// @lc code=end

