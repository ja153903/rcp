#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // perhaps the idea here is to do two passes
        // the first pass would be to initialize the set of unique items

        // then once we have the set of unique items we want to find a number num
        // in the set of numbers such that num - 1 does not exist within the set
        // if we find such a number, then we should increment that number until we find
        // the longest sequence
        let seen: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut longest: i32 = 0;

        for num in nums.iter() {
            if !seen.contains(&(num - 1)) {
                let mut curr = *num;
                let mut curr_len = 0;
                while seen.contains(&curr) {
                    curr_len += 1;
                    curr += 1;
                }

                longest = longest.max(curr_len);
            }
        }

        longest
    }
}
