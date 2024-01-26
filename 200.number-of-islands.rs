/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

struct Solution;

fn main() {
    let grid = vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1'],
    ];
    let ans = Solution::num_islands(grid);
    dbg!(ans);
}

// @lc code=start
use std::ops::Add;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Grid { grid }.num_cc() as i32
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

struct Seen {
    grid: Vec<Vec<bool>>,
}

impl Seen {
    fn new(dims: Point) -> Self {
        Self {
            grid: vec![vec![false; dims.col as usize]; dims.row as usize],
        }
    }

    fn contains(&self, p: Point) -> bool {
        self.grid[p.row as usize][p.col as usize]
    }

    fn insert(&mut self, p: Point) {
        self.grid[p.row as usize][p.col as usize] = true;
    }
}

impl Grid {
    fn num_cc(&self) -> usize {
        let mut count = 0;
        let mut seen = Seen::new(self.dims());
        for row in 0..self.dims().row {
            for col in 0..self.dims().col {
                let p = Point { row, col };
                if self.land(p) && !seen.contains(p) {
                    self.dfs(p, &mut seen);
                    count += 1;
                }
            }
        }
        count
    }

    fn dfs(&self, p: Point, seen: &mut Seen) {
        assert!(!seen.contains(p));
        seen.insert(p);

        for n in self.neighbors(p) {
            if self.land(n) && !seen.contains(n) {
                self.dfs(n, seen);
            }
        }
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

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }

    fn in_range(&self, p: Point) -> bool {
        let row = 0 <= p.row && p.row < self.dims().row;
        let col = 0 <= p.col && p.col < self.dims().col;
        row && col
    }

    fn land(&self, p: Point) -> bool {
        assert!(self.in_range(p));
        match self.grid[p.row as usize][p.col as usize] {
            '0' => false,
            '1' => true,
            c => panic!("not a binary digit: {c:?}"),
        }
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

