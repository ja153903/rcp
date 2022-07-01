#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() as i32) - 1;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = (mid as i32) + 1;
            } else {
                right = (mid as i32) - 1;
            }
        }

        -1
    }
}
