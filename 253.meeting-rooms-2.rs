/*
 * @lc app=leetcode id=253 lang=rust
 *
 * [253] Meeting Rooms 2
 */
use std::collections::BinaryHeap;
pub struct Solution;

// @lc code=start 
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut points = Vec::new();
        for interval in intervals {
            points.push((interval.start, 1));
            points.push((interval.end, -1));
        }

        points.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut meeting_rooms = 0;
        let mut ongoing_meetings = 0;
        for point in points {
            ongoing_meetings += point.1;
            meeting_rooms = max(meeting_rooms, ongoing_meetings);
        }

        meeting_rooms
    }
}
 // @lc code=end