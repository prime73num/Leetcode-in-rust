








// 344. Reverse String
// String 1

struct Solution {}
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let size = s.len();
        for i in 0..size/2 {
            let temp = s[i];
            s[i] = s[size-i-1];
            s[size-1-i] = temp;
        }
    }
}






fn main() {
}
