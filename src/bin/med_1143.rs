// 1143. Longest Common Subsequence
// Dynamic Programming 44
struct Solution {}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let bytes1 = text1.as_bytes();
        let bytes2 = text2.as_bytes();
        let mut dp = vec![0;bytes1.len()+1];
        for i in 0..bytes2.len() {
            let mut old = 0;
            let val = bytes2[i];
            for i in 1..dp.len() {
                let temp = dp[i];
                if dp[i-1] > dp[i] {
                    dp[i] = dp[i-1];
                }
                if val == bytes1[i-1] {
                    dp[i] = old + 1;
                }
                old = temp;
            }
        }
        return dp[bytes1.len()];
    }
}
fn main() {
}
