#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut frequency: HashMap<char, i32> = HashMap::new();
        let mut len: i32 = 0;

        let mut start: i32 = 0;
        let mut max_frequency: i32 = 0;

        for (end, ch) in s.chars().enumerate() {
            let end = end as i32;
            *frequency.entry(ch).or_insert(0) += 1;

            if let Some(&count) = frequency.get(&ch) {
                max_frequency = max_frequency.max(count);
            }

            // if the window size - the max frequency ~ number of replacements
            // this is greater than k, then this is invalid and we need to move
            // the sliding window up.
            while end - start + 1 - max_frequency > k {
                let u = start as usize;
                if let Ok(ch) = &s[u..u + 1].parse::<char>() {
                    *frequency.entry(*ch).or_insert(0) -= 1;
                }
                start += 1;
            }

            len = len.max(end - start + 1);
        }

        len
    }
}
