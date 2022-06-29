#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: i32 = 0;
        let mut j = (numbers.len() - 1) as i32;

        while i < j {
            let current = numbers[i as usize] + numbers[j as usize];
            if current == target {
                return vec![i + 1, j + 1];
            } else if current < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        vec![]
    }
}
