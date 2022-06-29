#![allow(dead_code)]
struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct BHNode {
    priority: i32,
    value: i32,
}

impl Ord for BHNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // to make this a min-heap, we should compare other against self
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for BHNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // to solve this problem, we need to count up the frequency of each number
        let mut result: Vec<i32> = Vec::new();
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut pq: BinaryHeap<BHNode> = BinaryHeap::new();

        for &num in nums.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }

        // if we want the k most frequent, then we have to remove the smaller ones
        // so we want a min heap in this case always keeping a size of k
        for (&value, &priority) in freq.iter() {
            pq.push(BHNode { priority, value });
            if pq.len() > k as usize {
                pq.pop();
            }
        }

        while !pq.is_empty() {
            if let Some(node) = pq.pop() {
                result.push(node.value);
            }
        }

        result
    }
}
