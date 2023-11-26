/*
 * @lc app=leetcode id=252 lang=rust
 *
 * [252] Meeting Rooms
 */

pub struct Solution;

// @lc code=start 
impl Solution {
    // @param intervals: an array of meeting time intervals
    // @return: if a person could attend all meetings
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        let mut intervals_copy = Vec::new();
        for interval in intervals {
            intervals_copy.push(Interval::new(interval.start, interval.end));
        }
        intervals_copy.sort_by(|a, b| a.start.cmp(&b.start));
        for window in intervals_copy.windows(2) {
            if window[0].end > window[1].start {
                return false;
            }
        }
        true
    } 
}
 // @lc code=end