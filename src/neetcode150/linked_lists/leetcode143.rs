#![allow(dead_code)]
struct Solution;

use crate::data_structures::linked_list::ListNode;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // 1. slow and fast pointer to get to middle
        // Note that we can't actually do fast and slow pointers
        // because in Rust, we cannot have two mutable references
        // to the same object
        // 2. reverse the second half
        // 3. merge algorithm!

        let len = Solution::get_length(&head);

        let mut ptr = head.as_mut();

        for _ in 0..(len / 2) {
            if let Some(p) = ptr {
                ptr = p.next.as_mut();
            }
        }

        if let Some(node) = ptr {
            let reverse = Solution::reverse(node.next.take(), None);

            if let Some(node) = head {
                node.next = Solution::merge(reverse, node.next.take(), len)
            }
        }
    }

    fn get_length(mut node: &Option<Box<ListNode>>) -> i32 {
        let mut count = 0;

        while let Some(n) = node {
            node = &n.next;
            count += 1;
        }

        count
    }

    fn reverse(head: Option<Box<ListNode>>, acc: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => acc,
            Some(mut node) => {
                let next = node.next;
                node.next = acc;

                Solution::reverse(next, Some(node))
            }
        }
    }

    fn merge(
        mut left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
        count: i32,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            None
        } else {
            match (left.as_mut(), right.as_ref()) {
                (None, None) => None,
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (Some(l), Some(_)) => {
                    l.next = Solution::merge(right, l.next.take(), count - 1);
                    left
                }
            }
        }
    }
}
