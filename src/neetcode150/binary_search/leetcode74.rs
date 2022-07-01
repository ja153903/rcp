#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // this problem can easily be solved with two binary searches
        // we first search on the rows, then within the columns
        // if we find a target row
        let mut left: i32 = 0;
        let mut right: i32 = (matrix.len() as i32) - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let last = *matrix[mid as usize].last().unwrap();
            let first = *matrix[mid as usize].first().unwrap();

            if first <= target && target <= last {
                let mut col_left = 0;
                let mut col_right = (matrix[mid as usize].len() as i32) - 1;

                while col_left <= col_right {
                    let col_mid = col_left + (col_right - col_left) / 2;
                    let value = matrix[mid as usize][col_mid as usize];

                    if value == target {
                        return true;
                    } else if value < target {
                        col_left = col_mid + 1;
                    } else {
                        col_right = col_mid - 1;
                    }
                }

                return false;
            } else if target > last {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}
