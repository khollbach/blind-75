// There's no leetcode submissions for this problem in Rust.

fn main() {
    // Ring graph of 4 nodes.
    let edges = vec![
        vec![1, 3],
        vec![0, 2],
        vec![1, 3],
        vec![0, 2],
    ];
    let mut nodes = HashMap::new();
    for i in 0..edges.len() as i32 {
        nodes.insert(i, Node::new(i));
    }
    for (i, outgoing) in (0..).zip(edges) {
        for j in outgoing {
            let other = Rc::clone(nodes.get(&j).unwrap());
            nodes.get_mut(&i).unwrap().borrow_mut().edges.push(other);
        }
    }

    let g = Graph { start: nodes.remove(&0).unwrap() };
    let _g2 = g.clone();
}

use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn clone_graph(g: Graph) -> Graph {
        g.clone()
    }
}

impl Clone for Graph {
    fn clone(&self) -> Self {
        fn dfs(curr: &Node, seen: &mut HashSet<usize>, out: &mut HashMap<usize, Link>) {
            let addr = curr.addr();
            if seen.contains(&addr) {
                return;
            }
            seen.insert(addr);

            // The first time we see a node, clone it.
            out.insert(addr, Node::new(curr.val));

            for e in &curr.edges {
                let other = &e.borrow();
                dfs(other, seen, out);

                // Insert the outgoing edge.
                let other = Rc::clone(out.get(&other.addr()).unwrap());
                let curr = out.get_mut(&curr.addr()).unwrap();
                curr.borrow_mut().edges.push(other);
            }
        }

        // Map from old node memory address to new node.
        let mut clones = HashMap::new();
        dfs(&self.start.borrow(), &mut HashSet::new(), &mut clones);

        let addr = self.start.borrow().addr();
        let new_start = clones.remove(&addr).unwrap();
        Self { start: new_start }
    }
}

/// A connected, undirected graph.
pub struct Graph {
    start: Link,
}

/// A pointer to a node.
type Link = Rc<RefCell<Node>>;

struct Node {
    val: i32,
    edges: Vec<Link>,
}

impl Node {
    fn new(val: i32) -> Link {
        Rc::new(RefCell::new(Self { val, edges: vec![] }))
    }

    fn addr(&self) -> usize {
        self as *const Self as usize
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        fn dfs(curr: Link, seen: &mut HashSet<usize>) {
            let addr = curr.borrow().addr();
            if seen.contains(&addr) {
                return;
            }
            seen.insert(addr);

            let edges = mem::take(&mut curr.borrow_mut().edges);
            for e in edges {
                dfs(e, seen);
            }
        }

        dfs(Rc::clone(&self.start), &mut HashSet::new())
    }
}
