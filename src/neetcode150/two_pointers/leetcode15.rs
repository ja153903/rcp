#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() - 2 {
            if i == 0 || nums[i] != nums[i - 1] {
                let mut j: i32 = (i + 1) as i32;
                let mut k: i32 = (nums.len() - 1) as i32;

                while j < k {
                    let current_value = nums[i] + nums[j as usize] + nums[k as usize];

                    if current_value == 0 {
                        result.push(vec![nums[i], nums[j as usize], nums[k as usize]]);

                        while j < k && nums[j as usize] == nums[(j + 1) as usize] {
                            j += 1;
                        }

                        while j < k && nums[k as usize] == nums[(k - 1) as usize] {
                            k -= 1;
                        }

                        j += 1;
                        k -= 1;
                    } else if current_value < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }

        result
    }
}
