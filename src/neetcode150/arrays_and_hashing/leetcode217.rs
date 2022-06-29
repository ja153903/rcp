#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut items = HashSet::new();

        for num in nums.iter() {
            if items.contains(num) {
                return true;
            }

            items.insert(num);
        }

        false
    }
}
