#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
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

        let pivot: i32 = left;

        left = 0;
        right = (nums.len() as i32) - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let real_mid = (mid + pivot) % (nums.len() as i32);

            if nums[real_mid as usize] == target {
                return real_mid;
            } else if nums[real_mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}
