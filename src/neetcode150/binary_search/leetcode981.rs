#![allow(dead_code)]

use std::collections::HashMap;

struct TimeMap {
    logs: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            logs: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.logs
            .entry(key)
            .or_insert_with(|| Vec::new())
            .push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(items) = self.logs.get(&key) {
            let mut left: i32 = 0;
            let mut right: i32 = items.len() as i32;

            while left < right {
                let mid = left + (right - left) / 2;

                if items[mid as usize].1 <= timestamp {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if right == 0 {
                String::from("")
            } else {
                items[right as usize - 1].0.to_string()
            }
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeMap;

    #[test]
    pub fn test_timemap() {
        let mut timemap = TimeMap::new();

        timemap.set(String::from("foo"), String::from("bar"), 1);

        assert_eq!(timemap.get(String::from("foo"), 1), String::from("bar"));
        assert_eq!(timemap.get(String::from("foo"), 3), String::from("bar"));

        timemap.set(String::from("foo"), String::from("bar2"), 4);

        assert_eq!(timemap.get(String::from("foo"), 4), String::from("bar2"));
    }
}
