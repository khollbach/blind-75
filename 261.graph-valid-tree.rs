/*
 * @lc app=leetcode id=261 lang=rust
 *
 * [261] Graph Valid Tree
 */

// @lc code=start
use std::convert::{identity, TryFrom, TryInto};

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = usize::try_from(n).unwrap();
        if n == 0 {
            return true; // idk
        }
        if edges.len() != n - 1 {
            return false;
        }

        let g = build_graph(n, edges);
        g.is_tree()
    }
}

impl Graph {
    fn is_tree(&self) -> bool {
        let n = self.nodes.len();
        let mut seen = vec![false; n];
        let mut to_visit = Vec::with_capacity(n);

        let start = 0;
        seen[start] = true;
        to_visit.push((start, start));

        while let Some((x, parent)) = to_visit.pop() {
            for &y in &self.nodes[x].edges {
                if y == parent {
                    continue;
                }
                if seen[y] {
                    return false; // detected a cycle
                }
                seen[y] = true;
                to_visit.push((y, x));
            }
        }

        let is_connected = seen.into_iter().all(identity);
        is_connected
    }
}

struct Graph {
    nodes: Vec<Node>,
}

#[derive(Default, Clone)]
struct Node {
    edges: Vec<usize>, // indices into nodes
}

fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> Graph {
    let mut nodes = vec![Node::default(); n];
    for e in edges {
        let [x, y] = e.try_into().unwrap();
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        nodes[x].edges.push(y);
        nodes[y].edges.push(x);
    }
    Graph { nodes }
}
// @lc code=end
