/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 */

struct Solution;

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_owned();
    let ans = Solution::exist(board, word);
    dbg!(ans);
}

// @lc code=start
use std::ops::Add;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        assert!(!word.is_empty());

        let grid = Grid { grid: board };
        for row in 0..grid.dims().row {
            for col in 0..grid.dims().col {
                if grid.dfs(Point { row, col }, &mut vec![], &word) {
                    return true
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    row: isize,
    col: isize,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn dfs(&self, curr: Point, prev: &mut Vec<Point>, word: &str) -> bool {
        if self.get(curr) != first(word) {
            return false;
        }
        prev.push(curr);

        let rest = rest(word);
        if rest.is_empty() {
            return true;
        }

        for p in self.neighbors(curr) {
            if !prev.contains(&p) && self.dfs(p, prev, rest) {
                return true;
            }
        }
        prev.pop();
        false
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item=Point> {
        let mut out = Vec::with_capacity(4);
        for d in DIRS {
            let p2 = p + d;
            if self.in_range(p2) {
                out.push(p2);
            }
        }
        out.into_iter()
    }

    fn in_range(&self, p: Point) -> bool {
        let row = p.row >= 0 && p.row < self.dims().row;
        let col = p.col >= 0 && p.col < self.dims().col;
        row && col
    }

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }

    fn get(&self, p: Point) -> char {
        assert!(self.in_range(p));
        self.grid[p.row as usize][p.col as usize]
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

fn first(word: &str) -> char {
    word.chars().next().unwrap()
}

fn rest(word: &str) -> &str {
    &word[1..]
}

// @lc code=end

