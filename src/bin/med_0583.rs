// 583. Delete Operation for Two Strings
// Dynamic Programming 49
struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();
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
        return (bytes1.len() + bytes2.len() - 2*dp[bytes1.len()]) as i32;
    }
}
fn main() {
}
