#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (height.len() as i32) - 1;

        let mut maxleft: i32 = height[left as usize];
        let mut maxright: i32 = height[right as usize];

        let mut result: i32 = 0;

        while left < right {
            if height[left as usize] < height[right as usize] {
                if height[left as usize] > maxleft {
                    maxleft = height[left as usize];
                } else {
                    result += maxleft - height[left as usize];
                }

                left += 1;
            } else {
                if height[right as usize] > maxright {
                    maxright = height[right as usize];
                } else {
                    result += maxright - height[right as usize];
                }

                right -= 1;
            }
        }

        result
    }
}
