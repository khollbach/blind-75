/*
 * @lc app=leetcode id=271 lang=rust
 *
 * [271] Encode and Decode Strings
 */

// @lc code=start
// Your Codec object will be instantiated and called as such:
// let obj = Codec::new();
// let s: String = obj.encode(strs);
// let ans: Vec<String> = obj.decode(s);

use std::mem;

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Self
    }
	
    pub fn encode(&self, strs: Vec<String>) -> String {
        let cap = 2 * strs.len() + strs.iter().map(|s| s.len()).sum::<usize>();
        let mut out = String::with_capacity(cap);

        for s in strs {
            for c in s.chars() {
                if c == '\0' {
                    // Escaped NUL.
                    out.push('\0');
                    out.push('\0');
                } else {
                    out.push(c);
                }
            }

            // End-of-string sequence.
            out.push('\0');
            out.push('\x7f');
        }

        out
    }
	
    pub fn decode(&self, s: String) -> Vec<String> {
        let mut out = vec![];
        let mut curr_string = String::new();

        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\0' {
                match chars.next().expect("unexpected EOF during escape sequence") {
                    // Escaped NUL.
                    '\0' => curr_string.push('\0'),
                    // End-of-string signal.
                    '\x7f' => {
                        out.push(mem::take(&mut curr_string));
                    }
                    d => panic!("invalid escape sequence: {c:?} {d:?}"),
                }
            } else {
                curr_string.push(c);
            }
        }

        assert!(curr_string.is_empty(), "unexpected EOF during string");
        out
    }
}
// @lc code=end

