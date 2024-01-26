/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

fn main() {
    // let n = 2;
    // let prereqs = vec![vec![0, 1]];
    // let ans = Solution::can_finish(n, prereqs);
    // dbg!(ans);

    let n = 4;
    let prereqs = vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]];
    let ans = Solution::can_finish(n, prereqs);
    dbg!(ans);
}

struct Solution;

// @lc code=start
use std::convert::{TryFrom, TryInto};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = usize::try_from(num_courses).unwrap();
        let g = build_graph(n, prerequisites);
        !g.has_cycle()
    }
}

impl Graph {
    fn has_cycle(&self) -> bool {
        let n = self.nodes.len();
        let mut seen = vec![false; n];
        let mut curr_path = vec![false; n];
        (0..n).any(|i| self.find_cycle(i, &mut seen, &mut curr_path))
    }

    fn find_cycle(&self, curr: usize, seen: &mut [bool], curr_path: &mut [bool]) -> bool {
        if curr_path[curr] {
            return true;
        }

        if seen[curr] {
            return false;
        }
        seen[curr] = true;

        curr_path[curr] = true;
        let ans = self.nodes[curr].edges.iter().any(|&other| self.find_cycle(other, seen, curr_path));
        curr_path[curr] = false;
        ans
    }
}

fn build_graph(n: usize, prereqs: Vec<Vec<i32>>) -> Graph {
    let mut nodes = vec![Node::default(); n];
    for ab in prereqs {
        let [a, b] = ab.try_into().unwrap();
        let a = usize::try_from(a).unwrap();
        let b = usize::try_from(b).unwrap();
        nodes[b].edges.push(a); // Add the edge: a <- b
    }
    Graph { nodes }
}

struct Graph {
    nodes: Vec<Node>,
}

#[derive(Default, Clone)]
struct Node {
    edges: Vec<usize>,
}
// @lc code=end
