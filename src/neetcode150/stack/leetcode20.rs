#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' => {
                    stack.push(')');
                }
                '[' => {
                    stack.push(']');
                }
                '{' => {
                    stack.push('}');
                }
                _ => {
                    if stack.is_empty() {
                        return false;
                    }

                    if stack.pop().unwrap() != ch {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
