#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // we can use a monotonic stack in this solution
        // the reason we want to use a variant of a monotonic stack
        // is that we want to grab the next greater element.
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::new();

        for (i, &temp) in temperatures.iter().enumerate() {
            // for each temperature, we want to check if its greater
            // than the top of the stack. If it is, then we should
            // pop the values from the stack and mark them within
            // the result list
            while !stack.is_empty() {
                if let Some(&prev_i) = stack.last() {
                    if temp > temperatures[prev_i] {
                        result[prev_i] = (i - prev_i) as i32;
                        stack.pop();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            stack.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_daily_temperatures() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];

        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }
}
