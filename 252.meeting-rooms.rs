/*
 * @lc app=leetcode id=252 lang=rust
 *
 * [252] Meeting Rooms
 */

pub struct Solution;

// @lc code=start 
impl Solution {
    pub fn can_attend_all_meetings(intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        for first, second in intervals.window(2) {
            if first[1] > second[0] {
                return false;
            }
        }
        true
    }
}
 // @lc code=end