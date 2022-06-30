#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let mut queue: VecDeque<(String, i32, i32)> = VecDeque::new();
        queue.push_back((String::from(""), 0, 0));

        while !queue.is_empty() {
            let (path, open, close) = queue.pop_front().unwrap();

            if path.len() as i32 == 2 * n {
                result.push(path)
            } else {
                if open < n {
                    queue.push_back((format!("{}(", path), open + 1, close));
                }

                if close < open {
                    queue.push_back((format!("{})", path), open, close + 1));
                }
            }
        }

        result
    }
}
