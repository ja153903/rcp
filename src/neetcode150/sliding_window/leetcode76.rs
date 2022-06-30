#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::from("");
        }

        let mut frequency: HashMap<char, i32> = HashMap::new();

        for ch in t.chars() {
            *frequency.entry(ch).or_insert(0) += 1;
        }

        let mut min_start: i32 = 0;
        let mut min_length: i32 = i32::MAX;
        let mut start: i32 = 0;
        let mut count: i32 = t.len() as i32;

        for (end, ch) in s.chars().enumerate() {
            let end = end as i32;
            if let Some(&freq) = frequency.get(&ch) {
                if freq > 0 {
                    count -= 1;
                }
            }

            *frequency.entry(ch).or_insert(0) -= 1;

            while count == 0 {
                if min_length > end - start + 1 {
                    min_start = start;
                    min_length = end - start + 1;
                }

                // while count is 0, then we increment our starting point
                let u = start as usize;
                let start_ch = &s[u..u + 1].parse::<char>().unwrap();
                if let Some(&freq) = frequency.get(start_ch) {
                    if freq >= 0 {
                        count += 1;
                    }
                }

                *frequency.entry(*start_ch).or_insert(0) += 1;
                start += 1;
            }
        }

        if min_length == i32::MAX {
            String::from("")
        } else {
            let min_start = min_start as usize;
            let min_length = min_length as usize;
            s[min_start..(min_start + min_length)].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_maximum_window_substring() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");

        let expected = String::from("BANC");

        assert_eq!(Solution::min_window(s, t), expected);
    }

    #[test]
    pub fn test_maximum_window_substring2() {
        let s = String::from("a");
        let t = String::from("b");

        let expected = String::from("");

        assert_eq!(Solution::min_window(s, t), expected);
    }
}
