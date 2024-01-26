/*
 * @lc app=leetcode id=417 lang=rust
 *
 * [417] Pacific Atlantic Water Flow
 */

// @lc code=start
use std::ops::Add;
use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(!heights.is_empty());
        let g = Grid { grid: heights };

        let cap = g.dims().row as usize + g.dims().col as usize;
        let mut pacific = Vec::with_capacity(cap);
        let mut atlantic = Vec::with_capacity(cap);
        for row in 0..g.dims().row {
            pacific.push(Point { row, col: 0 });
            atlantic.push(Point { row, col: g.dims().col - 1 });
        }
        for col in 0..g.dims().col {
            pacific.push(Point { row: 0, col });
            atlantic.push(Point { row: g.dims().row - 1, col });
        }

        let pacific = g.reachable(pacific);
        let atlantic = g.reachable(atlantic);
        pacific.intersection(&atlantic).map(|p| vec![p.row as i32, p.col as i32]).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn reachable(&self, start: Vec<Point>) -> HashSet<Point> {
        let mut seen: HashSet<_> = start.iter().copied().collect();
        let mut to_visit = start;
        while let Some(p) = to_visit.pop() {
            for n in self.neighbors(p) {
                if self.get(n) >= self.get(p) && !seen.contains(&n) {
                    seen.insert(n);
                    to_visit.push(n);
                }
            }
        }
        seen
    }

    fn get(&self, p: Point) -> i32 {
        assert!(self.in_range(p));
        self.grid[p.row as usize][p.col as usize]
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item=Point> {
        let mut out = Vec::with_capacity(4);
        for d in DIRS {
            if self.in_range(p + d) {
                out.push(p + d);
            }
        }
        out.into_iter()
    }

    fn in_range(&self, p: Point) -> bool {
        let row = 0 <= p.row && p.row < self.dims().row;
        let col = 0 <= p.col && p.col < self.dims().col;
        row && col
    }

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }
}

const DIRS: [Point; 4] = [
    Point { row: -1, col: 0 },
    Point { row: 1, col: 0 },
    Point { row: 0, col: -1 },
    Point { row: 0, col: 1 },
];

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

// @lc code=end

