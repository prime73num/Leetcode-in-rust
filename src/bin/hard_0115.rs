// 115. Distinct Subsequences
// Dynamic Programming 48
struct Solution {}
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let bytes_s = s.as_bytes();
        let bytes_t = t.as_bytes();
        let mut dp = vec![1;bytes_s.len()+1];
        let mut old = 1;
        dp[0] = 0;
        for i in bytes_t {
            for j in 1..dp.len() {
                let temp = dp[j];
                if bytes_s[j-1] == *i {
                    dp[j] = old + dp[j-1];
                } else {
                    dp[j] = dp[j-1];
                }
                old = temp;
            }
            old = dp[0];
        }
        return dp[bytes_s.len()];
    }
}
fn main() {
    Solution::num_distinct("ddd".to_string(), "dd".to_string());
}
