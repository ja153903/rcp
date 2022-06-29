#![allow(dead_code)]

#[derive(Default)]
struct Codec;

impl Codec {
    fn new() -> Self {
        Default::default()
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut encoded = String::new();

        for s in strs.iter() {
            encoded.push_str(format!("{}/{}", s.len(), s).as_str());
        }

        encoded
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut i: i32 = 0;
        let len = s.len() as i32;

        while i < len {
            let u = i as usize;
            if let Some(slash_index) = Codec::get_slash_index(&s, u) {
                let slice_len: usize = s[u..slash_index].parse::<usize>().unwrap();
                let slice = &s[(slash_index + 1)..(slash_index + slice_len + 1)];
                result.push(slice.to_string());
                i = (slash_index + slice_len + 1) as i32;
            } else {
                break;
            }
        }

        result
    }

    fn get_slash_index(s: &String, mut u: usize) -> Option<usize> {
        while u < s.len() {
            if &s[u..u + 1] == "/" {
                return Some(u);
            }
            u += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Codec;

    #[test]
    pub fn test_encode() {
        let codec = Codec::new();
        let result = codec.encode(vec![String::from("Hello")]);
        assert_eq!(result, String::from("5/Hello"));
    }

    #[test]
    pub fn test_decode() {
        let codec = Codec::new();
        let result = codec.decode(String::from("5/Hello"));
        assert_eq!(result, vec![String::from("Hello")]);
    }
}
