/*
 * @lc app=leetcode id=295 lang=rust
 *
 * [295] Find Median from Data Stream
 */

// @lc code=start
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
struct MedianFinder {
    left: BinaryHeap<i32>,
    middle: Option<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty() && self.middle.is_none()
    }
    
    fn add_num(&mut self, num: i32) {
        if self.is_empty() {
            self.middle = Some(num);
            return;
        }

        if let Some(mid) = self.middle.take() {
            let (a, b) = if num < mid { (num, mid) } else { (mid, num) };
            self.left.push(a);
            self.right.push(Reverse(b));
            return;
        }

        if num < *self.left.peek().unwrap() {
            self.middle = self.left.pop();
            self.left.push(num);
        } else if num > self.right.peek().unwrap().0 {
            self.middle = Some(self.right.pop().unwrap().0);
            self.right.push(Reverse(num));
        } else {
            self.middle = Some(num);
        }
    }
    
    fn find_median(&self) -> f64 {
        if let Some(mid) = self.middle {
            return mid as f64;
        }

        let a = *self.left.peek().unwrap();
        let b = self.right.peek().unwrap().0;
        (a as f64 + b as f64) / 2.
    }
}

// @lc code=end
