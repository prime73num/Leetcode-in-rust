










// 242. Valid Anagram
// Hash Map 2






struct Solution { }
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut char_map = [0;26];
        for i in s.as_bytes() {
            let idx = *i - b'a';
            char_map[idx as usize] += 1;
        }
        for i in t.as_bytes() {
            let idx = *i - b'a';
            char_map[idx as usize] -= 1;
        }
        return char_map.iter().all(|x| {
            return *x == 0;
        });
    }
}

fn main() {
}
