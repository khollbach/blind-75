/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        top_k_frequent(nums, k as usize)
    }
}

/// O(n)
fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();
    let num_to_freq = freqs(&nums);

    let mut freq_to_nums = HashMap::<_, Vec<i32>>::with_capacity(n);
    for (x, f) in num_to_freq {
        freq_to_nums.entry(f).or_default().push(x);
    }

    // Take the k most frequent.
    let mut out = Vec::with_capacity(k);
    for f in (0..=n).rev() {
        if let Some(xs) = freq_to_nums.get(&f) {
            for &x in xs {
                if out.len() == k {
                    return out;
                }
                out.push(x);
            }
        }
    }
    out
}

/// O(n lg n)
fn _top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let freqs = freqs(&nums);

    // Sort decreasing by freq.
    let mut elems: Vec<_> = freqs.into_iter().collect();
    elems.sort_unstable_by_key(|&(_, f)| Reverse(f));

    // Take k largest.
    elems.into_iter().take(k).map(|(x, _)| x).collect()
}

/// O(n)
fn freqs(nums: &[i32]) -> HashMap<i32, usize> {
    let mut freqs = HashMap::with_capacity(nums.len());
    for &x in nums {
        *freqs.entry(x).or_default() += 1;
    }
    freqs
}
// @lc code=end

