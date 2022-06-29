#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // we have to clean the string first such that we have all lower case chars
        let mut chars: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch.is_alphanumeric() {
                if let Some(lowercase_ch) = ch.to_lowercase().next() {
                    chars.push(lowercase_ch);
                }
            }
        }

        let mut i: i32 = 0;
        let mut j: i32 = (chars.len() as i32) - 1;

        while i < j {
            if chars[i as usize] != chars[j as usize] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}
