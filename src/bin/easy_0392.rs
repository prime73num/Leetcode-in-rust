// 392. Is Subsequence
// Dynamic Programming 47
struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let bytes_s = s.as_bytes();
        let bytes_t = t.as_bytes();
        let mut pointer = 0;
        for i in bytes_s {
            let mut find = false;
            while pointer < bytes_t.len() {
                if bytes_t[pointer] == *i {
                    pointer += 1;
                    find = true;
                    break;
                }
                pointer += 1;
            }
            if !find { return false;}
        }
        return true;
    }
}
fn main() {
}
