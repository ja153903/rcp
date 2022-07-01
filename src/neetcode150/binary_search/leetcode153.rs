#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() as i32) - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] > nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left as usize]
    }
}
