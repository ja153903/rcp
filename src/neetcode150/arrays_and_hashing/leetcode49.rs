#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let key = Solution::sort_word(s);
            groups.entry(key).or_insert(vec![]).push(s.clone());
        }

        groups.values().cloned().collect::<Vec<Vec<String>>>()
    }

    pub fn sort_word(s: &String) -> String {
        let mut frequency = [0; 26];

        for ch in s.chars() {
            let i = (ch as u8) - 97;
            frequency[i as usize] += 1;
        }

        let mut result = String::new();

        for i in 0..26 {
            let ch = ((i + 97) as u8) as char;
            for _ in 0..frequency[i] {
                result.push(ch);
            }
        }

        result
    }
}
