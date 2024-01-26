/*
 * @lc app=leetcode id=323 lang=rust
 *
 * [323] Number of Connected Components in an Undirected Graph
 */

// @lc code=start
use std::convert::{TryFrom, TryInto};

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = usize::try_from(n).unwrap();
        let g = build_graph(n, edges);
        g.count_cc() as i32
    }
}

impl Graph {
    fn count_cc(&self) -> usize {
        let n = self.nodes.len();
        let mut count = 0;
        let mut seen = vec![false; n];
        for i in 0..n {
            if !seen[i] {
                self.dfs(i, &mut seen);
                count += 1;
            }
        }
        count
    }

    fn dfs(&self, i: usize, seen: &mut [bool]) {
        let mut stack = vec![i];
        while let Some(i) = stack.pop() {
            for &j in &self.nodes[i].edges {
                if !seen[j] {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
    }

    // fn dfs(&self, i: usize, seen: &mut [bool]) {
    //     if seen[i] {
    //         return;
    //     }
    //     seen[i] = true;
    //     for &j in &self.nodes[i].edges {
    //         self.dfs(j, seen);
    //     }
    // }
}

fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> Graph {
    let mut nodes = vec![Node::default(); n];
    for ab in edges {
        let [a, b] = ab.try_into().unwrap();
        let a = usize::try_from(a).unwrap();
        let b = usize::try_from(b).unwrap();
        nodes[a].edges.push(b);
        nodes[b].edges.push(a);
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
