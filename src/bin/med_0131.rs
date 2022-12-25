











// 131. Palindrome Partitioning
// BackTrace 9
struct Solution {}
impl Solution {
    fn check(s: &[u8]) -> bool {
        let size = s.len();
        for i in 0..size {
            if s[i] != s[size-1-i] {
                return false;
            }
        }
        return true;
    }
    fn backtrace(pos: &mut Vec<String>, res: &mut Vec<Vec<String>>, s: &[u8]) {
        if s.len() == 0 {
            res.push(pos.clone());
            return;
        }
        let size = s.len();
        for i in 1..=size {
            let temp = &s[0..i];
            if Self::check(temp) {
                let cut = String::from_utf8_lossy(temp);
                pos.push(cut.to_string());
                Self::backtrace(pos, res, &s[i..]);
                pos.pop();
            }
        }
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut pos = Vec::new();
        let byte = s.as_bytes();
        Self::backtrace(&mut pos, &mut res, byte);
        res
    }
}
fn main() {
}
