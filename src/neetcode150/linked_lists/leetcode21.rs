#![allow(dead_code)]
struct Solution;

use crate::data_structures::linked_list::ListNode;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            None
        } else if list1.is_none() {
            list2
        } else if list2.is_none() {
            list1
        } else {
            let mut v1 = list1.unwrap();
            let mut v2 = list2.unwrap();

            if v1.val < v2.val {
                v1.next = Solution::merge_two_lists(v1.next, Some(v2));
                return Some(v1);
            } else {
                v2.next = Solution::merge_two_lists(Some(v1), v2.next);
                return Some(v2);
            }
        }
    }
}
