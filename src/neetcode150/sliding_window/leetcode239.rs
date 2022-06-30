#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        for (i, &num) in nums.iter().enumerate() {
            // lose all the items that are not within the window
            while !queue.is_empty() && (i - queue.front().unwrap()) as i32 > k - 1 {
                queue.pop_front();
            }

            // lose all the items in the back that cannot be maximums
            while !queue.is_empty() && num > nums[*queue.back().unwrap()] {
                queue.pop_back();
            }

            // insert the element
            queue.push_back(i);

            // add the element to our result list
            if i as i32 >= k - 1 {
                result.push(nums[*queue.front().unwrap()]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let expected = vec![3, 3, 5, 5, 6, 7];

        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }

    #[test]
    pub fn test_max_sliding_window2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];

        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }
}
