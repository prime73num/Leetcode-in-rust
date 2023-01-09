// 139. Word Break
// Dynamic Programming 26
struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false;s.len()+1];
        dp[0] = true;
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            for j in 0..=i {
                let temp = &bytes[j..=i];
                if dp[j] && word_dict.iter().find(|x| {
                    return x.as_bytes() == temp;
                }).is_some() {
                    dp[i + 1] = true;
                    break;
                }
            }
        }
        return dp[bytes.len()];
    }
}
fn main() {
}
