/*
 * @lc app=leetcode id=212 lang=rust
 *
 * [212] Word Search II
 */

struct Solution;

#[test]
fn main() {
    let grid = vec![
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
    ];

    let words = vec![
        "lllllll",
        "fffffff",
        "ssss",
        "s",
        "rr",
        "xxxx",
        "ttt",
        "eee",
        "ppppppp",
        "iiiiiiiii",
        "xxxxxxxxxx",
        "pppppp",
        "xxxxxx",
        "yy",
        "jj",
        "ccc",
        "zzz",
        "ffffffff",
        "r",
        "mmmmmmmmm",
        "tttttttt",
        "mm",
        "ttttt",
        "qqqqqqqqqq",
        "z",
        "aaaaaaaa",
        "nnnnnnnnn",
        "v",
        "g",
        "ddddddd",
        "eeeeeeeee",
        "aaaaaaa",
        "ee",
        "n",
        "kkkkkkkkk",
        "ff",
        "qq",
        "vvvvv",
        "kkkk",
        "e",
        "nnn",
        "ooo",
        "kkkkk",
        "o",
        "ooooooo",
        "jjj",
        "lll",
        "ssssssss",
        "mmmm",
        "qqqqq",
        "gggggg",
        "rrrrrrrrrr",
        "iiii",
        "bbbbbbbbb",
        "aaaaaa",
        "hhhh",
        "qqq",
        "zzzzzzzzz",
        "xxxxxxxxx",
        "ww",
        "iiiiiii",
        "pp",
        "vvvvvvvvvv",
        "eeeee",
        "nnnnnnn",
        "nnnnnn",
        "nn",
        "nnnnnnnn",
        "wwwwwwww",
        "vvvvvvvv",
        "fffffffff",
        "aaa",
        "p",
        "ddd",
        "ppppppppp",
        "fffff",
        "aaaaaaaaa",
        "oooooooo",
        "jjjj",
        "xxx",
        "zz",
        "hhhhh",
        "uuuuu",
        "f",
        "ddddddddd",
        "zzzzzz",
        "cccccc",
        "kkkkkk",
        "bbbbbbbb",
        "hhhhhhhhhh",
        "uuuuuuu",
        "cccccccccc",
        "jjjjj",
        "gg",
        "ppp",
        "ccccccccc",
        "rrrrrr",
        "c",
        "cccccccc",
        "yyyyy",
        "uuuu",
        "jjjjjjjj",
        "bb",
        "hhh",
        "l",
        "u",
        "yyyyyy",
        "vvv",
        "mmm",
        "ffffff",
        "eeeeeee",
        "qqqqqqq",
        "zzzzzzzzzz",
        "ggg",
        "zzzzzzz",
        "dddddddddd",
        "jjjjjjj",
        "bbbbb",
        "ttttttt",
        "dddddddd",
        "wwwwwww",
        "vvvvvv",
        "iii",
        "ttttttttt",
        "ggggggg",
        "xx",
        "oooooo",
        "cc",
        "rrrr",
        "qqqq",
        "sssssss",
        "oooo",
        "lllllllll",
        "ii",
        "tttttttttt",
        "uuuuuu",
        "kkkkkkkk",
        "wwwwwwwwww",
        "pppppppppp",
        "uuuuuuuu",
        "yyyyyyy",
        "cccc",
        "ggggg",
        "ddddd",
        "llllllllll",
        "tttt",
        "pppppppp",
        "rrrrrrr",
        "nnnn",
        "x",
        "yyy",
        "iiiiiiiiii",
        "iiiiii",
        "llll",
        "nnnnnnnnnn",
        "aaaaaaaaaa",
        "eeeeeeeeee",
        "m",
        "uuu",
        "rrrrrrrr",
        "h",
        "b",
        "vvvvvvv",
        "ll",
        "vv",
        "mmmmmmm",
        "zzzzz",
        "uu",
        "ccccccc",
        "xxxxxxx",
        "ss",
        "eeeeeeee",
        "llllllll",
        "eeee",
        "y",
        "ppppp",
        "qqqqqq",
        "mmmmmm",
        "gggg",
        "yyyyyyyyy",
        "jjjjjj",
        "rrrrr",
        "a",
        "bbbb",
        "ssssss",
        "sss",
        "ooooo",
        "ffffffffff",
        "kkk",
        "xxxxxxxx",
        "wwwwwwwww",
        "w",
        "iiiiiiii",
        "ffff",
        "dddddd",
        "bbbbbb",
        "uuuuuuuuu",
        "kkkkkkk",
        "gggggggggg",
        "qqqqqqqq",
        "vvvvvvvvv",
        "bbbbbbbbbb",
        "nnnnn",
        "tt",
        "wwww",
        "iiiii",
        "hhhhhhh",
        "zzzzzzzz",
        "ssssssssss",
        "j",
        "fff",
        "bbbbbbb",
        "aaaa",
        "mmmmmmmmmm",
        "jjjjjjjjjj",
        "sssss",
        "yyyyyyyy",
        "hh",
        "q",
        "rrrrrrrrr",
        "mmmmmmmm",
        "wwwww",
        "www",
        "rrr",
        "lllll",
        "uuuuuuuuuu",
        "oo",
        "jjjjjjjjj",
        "dddd",
        "pppp",
        "hhhhhhhhh",
        "kk",
        "gggggggg",
        "xxxxx",
        "vvvv",
        "d",
        "qqqqqqqqq",
        "dd",
        "ggggggggg",
        "t",
        "yyyy",
        "bbb",
        "yyyyyyyyyy",
        "tttttt",
        "ccccc",
        "aa",
        "eeeeee",
        "llllll",
        "kkkkkkkkkk",
        "sssssssss",
        "i",
        "hhhhhh",
        "oooooooooo",
        "wwwwww",
        "ooooooooo",
        "zzzz",
        "k",
        "hhhhhhhh",
        "aaaaa",
        "mmmmm",
    ];

    let expected = vec![
        "a",
        "aa",
        "aaa",
        "aaaa",
        "aaaaa",
        "aaaaaa",
        "aaaaaaa",
        "aaaaaaaa",
        "aaaaaaaaa",
        "aaaaaaaaaa",
    ];

    dbg!(grid.len());
    dbg!(grid[0].len());
    dbg!(words.iter().map(|s| s.len()).max());
    dbg!(expected.iter().map(|s| s.len()).max());

    let actual = Solution::find_words(grid, words.into_iter().map(str::to_owned).collect());

    let s1: HashSet<_> = expected.into_iter().map(str::to_owned).collect();
    let s2: HashSet<_> = actual.into_iter().collect();
    assert_eq!(s1, s2);
}

/*

*/

// @lc code=start
/*
idea:

* build a trie of the words
    - (as before)

* traverse every path thru the grid-graph, but stopping early when it's not a good prefix
    - every starting point
    - dfs
        - tracking the path so far (and not re-visiting those nodes)
        - popping up the stack
        - stopping early

Qs: de-dup output, or include multiples?

*/

use std::collections::{HashMap, HashSet};

struct Trie {
    nodes: Vec<Node>,
    root: usize,
}

#[derive(Default)]
struct Node {
    children: HashMap<char, usize>,
    word_ends_here: bool,
    parent: Option<usize>,
}

impl Trie {
    fn new(words: &[String]) -> Self {
        let mut this = Self::empty();
        for w in words {
            this.insert(w);
        }
        this
    }

    fn insert(&mut self, word: &str) {
        let mut curr = self.root;
        for c in word.chars() {
            curr = match self.nodes[curr].children.get(&c) {
                Some(&idx) => idx,
                None => {
                    let new_idx = self.nodes.len();
                    self.nodes.push(Node {
                        parent: Some(curr),
                        ..Node::default()
                    });
                    self.nodes[curr].children.insert(c, new_idx);
                    new_idx
                }
            };
        }
        self.nodes[curr].word_ends_here = true;
    }

    fn contains_prefix(&self, s: &str) -> bool {
        let mut curr = self.root;
        for c in s.chars() {
            let Some(&next) = self.nodes[curr].children.get(&c) else {
                return false;
            };
            curr = next;
        }
        true
    }

    fn contains_word(&self, word: &str) -> bool {
        let mut curr = self.root;
        for c in word.chars() {
            let Some(&next) = self.nodes[curr].children.get(&c) else {
                return false;
            };
            curr = next;
        }
        self.nodes[curr].word_ends_here
    }

    fn empty() -> Self {
        Self {
            nodes: vec![Node::default()],
            root: 0,
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let board = Board::new(board);
        let trie = Trie::new(&words);
        let mut out = HashSet::new();

        for row in 0..board.dims().row {
            for col in 0..board.dims().col {
                let p = Point { row, col };
                let c = board.get(p);
                let Some(&trie_cursor) = trie.nodes[trie.root].children.get(&c) else {
                    continue;
                };
                board.dfs(&trie, trie_cursor, &mut vec![p], &mut out);
            }
        }

        out.into_iter().collect()
    }
}

impl Board {
    fn dfs(
        &self,
        trie: &Trie,
        mut trie_cursor: usize,
        curr_path: &mut Vec<Point>,
        out: &mut HashSet<String>,
    ) {
        if trie.nodes[trie_cursor].word_ends_here {
            let s = curr_path.iter().map(|&p| self.get(p)).collect();
            out.insert(s);
        }

        let curr = *curr_path.last().unwrap();
        for p in self.neighbors(curr) {
            if curr_path.contains(&p) {
                continue;
            }
            let c = self.get(p);
            let Some(&new_cursor) = trie.nodes[trie_cursor].children.get(&c) else {
                continue;
            };

            trie_cursor = new_cursor;
            curr_path.push(p);

            self.dfs(trie, trie_cursor, curr_path, out);

            curr_path.pop();
            trie_cursor = trie.nodes[trie_cursor].parent.unwrap();
        }
    }

    fn neighbors(&self, p: Point) -> Vec<Point> {
        let mut out = vec![];
        for (row, col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let p2 = Point {
                row: p.row + row,
                col: p.col + col,
            };
            if self.in_bounds(p2) {
                out.push(p2);
            }
        }
        out
    }

    fn new(grid: Vec<Vec<char>>) -> Self {
        // Empty?
        assert!(!grid.is_empty());
        assert!(!grid[0].is_empty());

        // Jagged?
        for row in &grid {
            assert_eq!(row.len(), grid[0].len());
        }

        Self { grid }
    }

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }

    fn in_bounds(&self, p: Point) -> bool {
        let row = 0 <= p.row && p.row < self.dims().row;
        let col = 0 <= p.col && p.col < self.dims().col;
        row && col
    }

    fn get(&self, p: Point) -> char {
        assert!(self.in_bounds(p));
        self.grid[p.row as usize][p.col as usize]
    }
}

struct Board {
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}
// @lc code=end
