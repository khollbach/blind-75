/*
 * @lc app=leetcode id=269 lang=rust
 *
 * [269] Alien Dictionary
 */

 /*
 approach:
  - for each window of size 2, insert an edge b/w a pair of letters
  - top sort the letter graph (validating its a dag)
  */

struct Solution;

fn main() {
    let words = ["wrt","wrf","er","ett","rftt"].into_iter().map(str::to_owned).collect();
    let expected = "wertf";
    let ans = Solution::alien_order(words);
    assert_eq!(ans, expected);
}

// @lc code=start
use std::iter::zip;
use std::fmt;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let Ok(g) = Graph::from_words(words) else {
            return String::new();
        };
        g.top_sort().unwrap_or(String::new())
    }
}

impl Graph {
    fn top_sort(&self) -> Option<String> {
        let mut order = String::with_capacity(NUM_CHARS);

        let mut in_degrees = [0u8; NUM_CHARS];
        for x in self.nodes() {
            for y in x.edges() {
                in_degrees[y] += 1;
            }
        }

        let mut exposed = Vec::with_capacity(26);
        for x in self.labels() {
            if in_degrees[x] == 0 {
                exposed.push(x);
            }
        }

        while let Some(x) = exposed.pop() {
            order.push(alpha(x));
            for y in self.get(x).edges() {
                in_degrees[y] -= 1;
                if in_degrees[y] == 0 {
                    exposed.push(y);
                }
            }
        }

        if in_degrees.iter().all(|&d| d == 0) {
            Some(order)
        } else {
            None
        }
    }
}

const NUM_CHARS: usize = 26;

#[derive(Default)]
struct Graph {
    nodes: [Option<Node>; NUM_CHARS],
}

#[derive(Default)]
struct Node {
    edges: u32, // bitset of 26 bools
}

impl Node {
    fn edges(&self) -> impl Iterator<Item=usize> + '_ {
        (0..NUM_CHARS).filter(move |&idx| self.has_edge(idx))
    }

    fn has_edge(&self, idx: usize) -> bool {
        self.edges & 1 << idx != 0
    }

    fn insert_edge(&mut self, idx: usize) {
        self.edges |= 1 << idx;
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "edges: {:b}", self.edges)
    }
}

impl Graph {
    fn get(&self, idx: usize) -> &Node {
        self.nodes[idx].as_ref().unwrap()
    }

    fn labels(&self) -> impl Iterator<Item=usize> + '_ {
        (0..NUM_CHARS).filter(move |&i| self.nodes[i].is_some())
    }

    fn nodes(&self) -> impl Iterator<Item=&Node> + '_ {
        (0..NUM_CHARS).filter_map(move |i| self.nodes[i].as_ref())
    }

    fn new() -> Self {
        Self::default()
    }

    fn from_words(words: Vec<String>) -> Result<Self, BuildGraphError> {
        let mut this = Self::new();

        this.insert_nodes(&words);

        for st in words.windows(2) {
            let [s, t] = st else { unreachable!() };
            this.insert_edge(s, t)?;
        }

        Ok(this)
    }

    fn insert_nodes(&mut self, words: &[String]) {
        for w in words {
            for c in w.chars() {
                let x = idx(c);
                if self.nodes[x].is_none() {
                    self.nodes[x] = Some(Node::default());
                }
            }
        }
    }

    fn insert_edge(&mut self, s: &str, t: &str) -> Result<(), BuildGraphError> {
        for (c, d) in zip(s.chars(), t.chars()) {
            if c != d {
                let x = idx(c);
                let y = idx(d);
                self.nodes[x].as_mut().unwrap().edges |= 1 << y;
                return Ok(());
            }
        }

        if s.len() <= t.len() {
            Ok(())
        } else {
            Err(BuildGraphError::InvalidOrder)
        }
    }
}

fn idx(c: char) -> usize {
    let offset = c as u8 - b'a';
    offset as usize
}

fn alpha(idx: usize) -> char {
    assert!(idx < NUM_CHARS);
    let ascii = b'a' + idx as u8;
    ascii as char
}

enum BuildGraphError {
    InvalidOrder,
}
// @lc code=end
