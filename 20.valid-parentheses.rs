/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution;

// @lc code=start
impl Solution {
    /// O(n)
    pub fn is_valid(s: String) -> bool {
        let mut open = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => open.push(c),

                ')' | ']' | '}' => {
                    let expected = close_to_open(c);
                    if open.pop() != Some(expected) {
                        return false;
                    }
                }

                _ => panic!("unexpected symbol: {c:?}"),
            }
        }

        open.is_empty()
    }
}

fn close_to_open(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("not a closing symbol: {c:?}"),
    }
}
// @lc code=end

