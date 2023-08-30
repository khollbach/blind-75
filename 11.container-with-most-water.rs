/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        max_area(&height)
    }
}

use std::cmp::{max, Reverse};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// Lowering water-level idea: if you consider lines in order of decreasing height,
/// you only need to remember the left-most and right-most lines you've seen so far.
///
/// O(n lg n)
pub fn water_level(heights: &[i32]) -> i32 {
    let n = heights.len();
    if n == 0 {
        return 0;
    }

    // Tag with x-position, and sort decreasing by height.
    let mut heights: Vec<_> = heights
        .iter()
        .enumerate()
        .map(|(i, &h)| Point { x: i as i32, y: h })
        .collect();
    heights.sort_unstable_by_key(|&p| Reverse(p.y));

    let mut max_area = 0;
    let mut leftmost_seen = heights[0];
    let mut rightmost_seen = heights[0];

    for i in 1..n {
        let curr = heights[i];

        for point in [leftmost_seen, rightmost_seen] {
            max_area = max(max_area, curr.y * (curr.x - point.x).abs());
        }

        if curr.x < leftmost_seen.x {
            leftmost_seen = curr;
        }
        if curr.x > rightmost_seen.x {
            rightmost_seen = curr;
        }
    }

    max_area
}

use std::cmp::min;

/// O(n) -- turns out you can do a greedy thing from both ends.
fn max_area(heights: &[i32]) -> i32 {
    if heights.is_empty() {
        return 0;
    }

    let mut max_area = 0;

    let mut i = 0;
    let mut j = heights.len() - 1;
    while i < j {
        max_area = max(max_area, (j - i) as i32 * min(heights[i], heights[j]));

        // Throw out the shorter one; it'll never be more useful than it was just now.
        if heights[i] < heights[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    max_area
}
// @lc code=end
