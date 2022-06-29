#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen: HashMap<char, i32> = HashMap::new();
        // we want to use a hashmap to keep track of previous indices
        // that we've seen.
        let mut start: i32 = 0;
        let mut max: i32 = 0;

        for (i, ch) in s.chars().enumerate() {
            // if we've seen the character before
            // we need to update our start window
            // now this value will be the max between
            // the current start index or the previous
            // index we've seen the character incremented
            // by 1.
            // the reason we do this is because the previous
            // index we've seen could be less than the start
            // that we're currently on. In this case, we know
            // that we shouldn't update the start index just yet
            if let Some(&prev_idx) = seen.get(&ch) {
                start = start.max(prev_idx + 1);
            }

            seen.insert(ch, i as i32);
            max = max.max((i as i32) - start + 1);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }
}
