//! There's no leetcode starter code for this problem. I think using Rc/RefCell
//! for shared mutable heap allocations makes the most sense.
//! 
//! We could also try raw pointers: `struct Node { val: i32, next: *mut Node }`.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct List {
    head: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    #[allow(unused)]
    value: i32,
    next: Link,
}

impl List {
    pub fn has_cycle(&self) -> bool {
        let mut seen = HashSet::new();

        let mut next = self.head.clone();
        while let Some(curr) = next {
            // Already seen?
            let addr = Rc::as_ptr(&curr);
            if seen.contains(&addr) {
                return true;
            }
            seen.insert(addr);

            next = curr.borrow().next.clone();
        }

        false
    }

    pub fn has_cycle_2(&self) -> bool {
        self.find_cycle().is_some()
    }

    /// Return a node in the cycle.
    fn find_cycle(&self) -> Link {
        let mut fast = self.head.clone()?;
        let mut slow = self.head.clone()?;

        loop {
            let tmp = fast.borrow().next.clone()?.borrow().next.clone()?;
            fast = tmp;
            let tmp = slow.borrow().next.clone()?;
            slow = tmp;

            if Rc::ptr_eq(&fast, &slow) {
                return Some(Rc::clone(&fast));
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if let Some(node) = self.find_cycle() {
            // Break the cycle.
            node.borrow_mut().next = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[3, 2, 0, 4], Some(1))]
    #[test_case(&[1, 2], Some(0))]
    #[test_case(&[1], Some(0))]
    #[test_case(&[1], None)]
    #[test_case(&[], None)]
    fn test_cases(nums: &[i32], tail_pointer: Option<usize>) {
        let list = match tail_pointer {
            Some(i) => List::with_cycle(nums, i),
            None => List::from_slice(nums),
        };

        let has_cycle = tail_pointer.is_some();
        assert_eq!(has_cycle, list.has_cycle());
        assert_eq!(has_cycle, list.has_cycle_2());
    }

    impl List {
        fn empty() -> List {
            List { head: None }
        }

        fn from_slice(values: &[i32]) -> List {
            let mut list = List::empty();
            for &value in values.iter().rev() {
                list.push(value)
            }
            list
        }

        fn with_cycle(values: &[i32], cycle_start: usize) -> List {
            assert!(cycle_start < values.len());

            let list = List::from_slice(values);

            let node = list.get_node(cycle_start).unwrap();
            let tail = list.tail_node().unwrap();
            tail.borrow_mut().next = Some(node);

            list
        }

        fn len(&self) -> usize {
            let mut count = 0;
            let mut next = self.head.clone();
            while let Some(curr) = next {
                count += 1;
                next = curr.borrow().next.clone();
            }
            count
        }

        fn push(&mut self, value: i32) {
            self.head = Some(Rc::new(RefCell::new(Node {
                value,
                next: self.head.take(),
            })));
        }

        fn get_node(&self, index: usize) -> Link {
            let mut curr = self.head.clone()?;
            for _ in 0..index {
                let tmp = curr.borrow().next.clone()?;
                curr = tmp;
            }
            Some(curr)
        }

        fn tail_node(&self) -> Link {
            let n = self.len();
            self.get_node(n - 1)
        }
    }
}
