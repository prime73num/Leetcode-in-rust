// 3. Longest Substring Without Repeating Characters

use std::collections::hash_set;
struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut res = 0;
        let mut set = hash_set::HashSet::new();
        let mut next = 0;
        let mut pre = 0;
        set.insert(pre);
        for i in 0..bytes.len() {
            set.remove(&pre);
            while next < bytes.len() && !set.contains(&bytes[next]) {
                set.insert(bytes[next]);
                next += 1;
            }
            let temp = next - i;
            if temp > res {
                res = temp;
            }
            if next == bytes.len() {
                return res as i32;
            }
            pre = bytes[i];
        }
        return res as i32;
    }
}
fn main() {
}
