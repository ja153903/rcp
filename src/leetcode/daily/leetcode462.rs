#![allow(dead_code)]
struct Solution;

// https://math.stackexchange.com/questions/113270/the-median-minimizes-the-sum-of-absolute-deviations-the-ell-1-norm
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mid: usize = nums.len() / 2;

        let median = if nums.len() % 2 == 0 {
            (nums[mid] + nums[mid - 1]) / 2
        } else {
            nums[mid]
        };

        nums.iter().map(|num| (num - median).abs()).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_min_moves2() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    }
}
