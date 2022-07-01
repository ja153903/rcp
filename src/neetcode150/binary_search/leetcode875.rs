#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = *piles.iter().max().unwrap();
        let mut result: i32 = right;

        while left < right {
            let mid = left + (right - left) / 2;
            // if can finish, mark results and then
            if Solution::can_finish(&piles, h, mid) {
                result = result.min(mid);
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        result
    }

    fn can_finish(piles: &Vec<i32>, h: i32, k: i32) -> bool {
        let mut hours_needed: i32 = 0;

        for &pile in piles.iter() {
            hours_needed += pile / k;
            if pile % k > 0 {
                hours_needed += 1;
            }
        }

        hours_needed <= h
    }
}
