/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

pub struct Solution;

// @lc code=start
use std::collections::{HashMap, BTreeMap};

impl Solution {
    /// O(n), assuming alphabet size is 26.
    /// Here "n" is sum(len(s) for s in strs).
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::<_, Vec<String>>::with_capacity(strs.len());

        for s in strs {
            let freqs = freqs(&s);

            // hashing `freqs` is O(alphabet_size) -- 26 in our case
            groups.entry(freqs).or_default().push(s);
        }

        groups.into_iter().map(|(_freqs, anagrams)| anagrams).collect()
    }
}

/// O(n) -- assuming the alphabet is limited to `a..=z`.
fn freqs(s: &str) -> BTreeMap<char, usize> {
    let mut freqs = BTreeMap::new();
    for c in s.chars() {
        *freqs.entry(c).or_default() += 1;
    }
    freqs
}
// @lc code=end

