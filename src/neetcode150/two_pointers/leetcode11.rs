#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut left: i32 = 0;
        let mut right: i32 = (height.len() - 1) as i32;
        let mut max: i32 = 0;

        while left < right {
            let area = height[left as usize].min(height[right as usize]) * (right - left);
            max = max.max(area);

            if height[left as usize] < height[right as usize] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max
    }
}
