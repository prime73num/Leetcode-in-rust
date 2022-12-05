














// 1002. Find Common Characters
// Hash Map 3


struct Solution {}
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut char_map = [std::i32::MAX;26];
        let mut res = vec![];
        for s in &words {
            let mut temp_map = [0;26];
            for c in s.as_bytes() {
                let idx = c - b'a';
                temp_map[idx as usize] += 1;
            }
            char_map.iter_mut().enumerate().for_each(|(x, y)| {
                if temp_map[x] < *y {
                    *y = temp_map[x];
                }
            });
        }
        char_map.iter().enumerate().for_each(|(x,y)| {
            for _ in 0..*y {
                let mut s = String::new();
                s.push((x as u8 + b'a') as char);
                res.push(s)
            }
        });
        res
    }
}

fn main() {
}
