// 647. Palindromic Substrings
// Dynamic Programming 53
struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut res = 0;
        let mut start = 0 as i32;
        let mut end = 0 as i32;
        let bytes = s.as_bytes();
        let size = bytes.len();
        for i in 0..2*size-1{
            start = (i / 2) as i32;
            end = i as i32 - start;
            while start >= 0 && end < size as i32 {
                if bytes[start as usize] == bytes[end as usize] {
                    res += 1;
                } else { break; }
                start -= 1;
                end += 1;
            }
        }
        return res;
    }
}
fn main() {
}
