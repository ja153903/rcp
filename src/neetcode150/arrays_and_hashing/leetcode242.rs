#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut counter: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(count) = counter.get_mut(&ch) {
                if *count == 0 {
                    return false;
                }

                *count -= 1;
            } else {
                return false;
            }
        }

        counter.values().sum::<i32>() == 0
    }
}
