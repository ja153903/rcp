#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // keep track of values within a stack
        // when we hit an operation, then we know
        // that we should pop two values from the stack
        // and perform some operation on them
        let mut stack: Vec<String> = Vec::new();
        let operations: HashSet<&str> =
            HashSet::from_iter(vec!["+", "-", "*", "/"].iter().cloned());

        for token in tokens.iter() {
            if operations.contains(token.as_str()) {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let b_as_i32 = b.parse::<i32>().unwrap();
                let a_as_i32 = a.parse::<i32>().unwrap();

                let result: i32 = match token.as_str() {
                    "+" => a_as_i32 + b_as_i32,
                    "-" => a_as_i32 - b_as_i32,
                    "*" => a_as_i32 * b_as_i32,
                    "/" => a_as_i32 / b_as_i32,
                    _ => 0,
                };

                stack.push(result.to_string());
            } else {
                stack.push(token.to_string());
            }
        }

        stack.last().unwrap().parse::<i32>().unwrap()
    }
}
