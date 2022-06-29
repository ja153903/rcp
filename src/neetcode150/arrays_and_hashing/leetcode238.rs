#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut right = 1;

        for i in (0..nums.len()).rev() {
            result[i] *= right;
            right *= nums[i];
        }

        result
    }
}
