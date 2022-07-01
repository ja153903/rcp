#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut result: i32 = 0;

        for (i, &num) in heights.iter().enumerate() {
            let mut start: usize = i;
            while !stack.is_empty() && stack.last().unwrap().1 > num {
                if let Some(last) = stack.pop() {
                    result = result.max(last.1 * (i - last.0) as i32);
                    start = last.0;
                }
            }
            stack.push((start, num));
        }

        for tuple in stack.iter() {
            result = result.max(tuple.1 * (heights.len() - tuple.0) as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
        // assert_eq!(Solution::largest_rectangle_area(vec![4, 4, 4]), 12);
    }
}
