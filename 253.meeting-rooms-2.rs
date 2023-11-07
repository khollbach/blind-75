/*
 * @lc app=leetcode id=253 lang=rust
 *
 * [253] Meeting Rooms 2
 */
use std::collections::BinaryHeap;
pub struct Solution;

// @lc code=start 
impl Solution {
    pub fn maximum_meetings(intervals: Vec<Vec<i32>>) -> i32 {
        let mut end_point = BinaryHeap::new();
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        for interval in intervals {
            if let Some(priority) = end_point.pop() {
                if abs(priority) <= interval[1] {
                    end_point.push(priority - interval[1]);
                } else {
                    end_point.push(priority);
                    end_point.push(-interval[1]);
                }
            } else {
                end_point.push(-interval[1]);
            }
        }
        end_point.len() as i32
    }
}
 // @lc code=end