/*
 * @lc app=leetcode id=178 lang=rust
 *
 * [###] Is the graph a valid tree?
 * https://www.lintcode.com/problem/178/
 */
pub struct Solution;

#[derive(Clone)]
pub struct Node {
    pub neighbours: Vec<usize>,
}

// @lc code=start 
impl Solution {
    // @param n: An integer
    // @param edges: a list of undirected edges
    // @return: true if it's a valid tree, or false
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![Node{ neighbours: vec![] }; n as usize];
        for edge in edges {
            graph[edge[0] as usize].neighbours.push(edge[1] as usize);
            graph[edge[1] as usize].neighbours.push(edge[0] as usize);
        }

        let mut visited = vec![false; n as usize];
        if !dfs(&mut visited, &graph, 0) {
            return false;
        }
        
        visited.iter().all(|&x| x)
    }
}

pub fn dfs(visited: &mut Vec<bool>, graph: &Vec<Node>, node: usize) -> bool {
    let mut stack = vec![(node, node)];
    while !stack.is_empty() {
        let (parent, current) = stack.pop().unwrap();
        visited[current] = true;
        for neighbour in &graph[current].neighbours {
            if *neighbour != parent {
                if visited[*neighbour] {
                    return false;
                }
                stack.push((current, *neighbour))
            }
        }
    }
    true
}
 // @lc code=end

