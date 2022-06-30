#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let k: i32 = s1.len() as i32;
        let mut count = k;
        let mut freq = vec![0; 26];
        let mut start: i32 = 0;

        for ch in s1.chars() {
            freq[(ch as u8 - 97) as usize] += 1;
        }

        for (end, ch) in s2.chars().enumerate() {
            let index: usize = ((ch as u8) - 97) as usize;
            let end = end as i32;

            if freq[index] > 0 {
                count -= 1;
            }

            freq[index] -= 1;

            if count == 0 {
                return true;
            }

            // here we use if instead of while like in other
            // sliding window problems because we have to have
            // it equal to k before we switch out the starting pos
            if end - start + 1 == k {
                let u = start as usize;
                let ch = &s2[u..u + 1].parse::<char>().unwrap();
                let ch_as_index = ((*ch as u8) - 97) as usize;

                if freq[ch_as_index] >= 0 {
                    count += 1;
                }

                freq[ch_as_index] += 1;
                start += 1;
            }
        }

        false
    }
}
