/*
 * @lc app=leetcode id=??? lang=rust
 *
 * [323] Number of Connected Components (undirected)
 * https://www.lintcode.com/problem/3651/
 */
pub struct Solution;

#[derive(Clone)]
pub struct Node {
    pub neighbours: Vec<usize>,
}

// @lc code=start

impl Solution {
    // @param n: the number of vertices
    // @param edges: the edges of undirected graph
    // @return: the number of connected components
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![Node{ neighbours: vec![] }; n as usize];
        for edge in edges {
            graph[edge[0] as usize].neighbours.push(edge[1] as usize);
            graph[edge[1] as usize].neighbours.push(edge[0] as usize);
        }

        let mut visited = vec![false; n as usize];
        let mut num_components = 0;
        for i in 0..(n as usize) {
            if !visited[i] {
                dfs(&mut visited, &graph, i);
                num_components += 1;
            }
        }
        num_components
    }
}

pub fn dfs(visited: &mut Vec<bool>, graph: &Vec<Node>, node_idx: usize) {
    let mut stack = vec![node_idx];
    while !stack.is_empty() {
        let node_idx = stack.pop().unwrap();
        visited[node_idx] = true;
        for i in &graph[node_idx].neighbours {
            if !visited[*i] {
                stack.push(*i)
            }
        }
    }
}
// @lc code=end