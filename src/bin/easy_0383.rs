












// 383. Ransom Note
// Hash Map 8










struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_map = [0;26];
        for i in magazine.as_bytes() {
            let idx = *i - b'a';
            char_map[idx as usize] += 1;
        }
        for i in ransom_note.as_bytes() {
            let idx = *i - b'a';
            char_map[idx as usize] -= 1;
        }
        return char_map.iter().all(|x| {
            return *x >= 0;
        });
    }
}
fn main() {
}
