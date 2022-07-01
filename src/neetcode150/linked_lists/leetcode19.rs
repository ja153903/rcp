#![allow(dead_code)]

use crate::data_structures::linked_list::ListNode;
struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut start = Box::new(ListNode { next: head, val: 0 });

        let mut right = start.clone();
        let mut left = start.as_mut();

        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();

        start.next
    }
}
